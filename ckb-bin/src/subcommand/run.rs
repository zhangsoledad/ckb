use crate::helper::{deadlock_detection, wait_for_exit};
use ckb_app_config::{BlockAssemblerConfig, ExitCode, RunArgs};
use ckb_build_info::Version;
use ckb_chain::chain::ChainService;
use ckb_jsonrpc_types::ScriptHashType;
use ckb_logger::info_target;
use ckb_network::{
    CKBProtocol, NetworkService, NetworkState, MAX_FRAME_LENGTH_ALERT, MAX_FRAME_LENGTH_RELAY,
    MAX_FRAME_LENGTH_SYNC, MAX_FRAME_LENGTH_TIME,
};
use ckb_network_alert::alert_relayer::AlertRelayer;
use ckb_resource::Resource;
use ckb_rpc::{RpcServer, ServiceBuilder};
use ckb_shared::shared::{Shared, SharedBuilder};
use ckb_sync::{NetTimeProtocol, NetworkProtocol, Relayer, SyncShared, Synchronizer};
use ckb_types::prelude::*;
use ckb_util::{Condvar, Mutex};
use ckb_verification::{GenesisVerifier, Verifier};
use std::sync::Arc;

const SECP256K1_BLAKE160_SIGHASH_ALL_ARG_LEN: usize = 20;

pub fn run(args: RunArgs, version: Version) -> Result<(), ExitCode> {
    deadlock_detection();

    let block_assembler_config = sanitize_block_assembler_config(&args)?;
    let miner_enable = block_assembler_config.is_some();
    let exit_condvar = Arc::new((Mutex::new(()), Condvar::new()));

    let (shared, table) = SharedBuilder::with_db_config(&args.config.db)
        .consensus(args.consensus)
        .tx_pool_config(args.config.tx_pool)
        .notify_config(args.config.notify)
        .store_config(args.config.store)
        .block_assembler_config(block_assembler_config)
        .build()
        .map_err(|err| {
            eprintln!("Run error: {:?}", err);
            ExitCode::Failure
        })?;

    // Verify genesis every time starting node
    verify_genesis(&shared)?;

    set_system_cell(&shared);

    ckb_memory_tracker::track_current_process(args.config.memory_tracker.interval);

    let chain_service = ChainService::new(shared.clone(), table);
    let chain_controller = chain_service.start(Some("ChainService"));
    info_target!(crate::LOG_TARGET_MAIN, "ckb version: {}", version);
    info_target!(
        crate::LOG_TARGET_MAIN,
        "chain genesis hash: {:#x}",
        shared.genesis_hash()
    );

    let sync_shared = Arc::new(SyncShared::new(shared.clone()));
    let network_state = Arc::new(
        NetworkState::from_config(args.config.network).expect("Init network state failed"),
    );
    let synchronizer = Synchronizer::new(chain_controller.clone(), Arc::clone(&sync_shared));

    let relayer = Relayer::new(
        chain_controller.clone(),
        Arc::clone(&sync_shared),
        args.config.tx_pool.min_fee_rate,
        args.config.tx_pool.max_tx_verify_cycles,
    );
    let net_timer = NetTimeProtocol::default();
    let alert_signature_config = args.config.alert_signature.unwrap_or_default();
    let alert_relayer = AlertRelayer::new(
        version.to_string(),
        shared.notify_controller().clone(),
        alert_signature_config,
    );

    let alert_notifier = Arc::clone(alert_relayer.notifier());
    let alert_verifier = Arc::clone(alert_relayer.verifier());

    let synchronizer_clone = synchronizer.clone();
    let protocols = vec![
        CKBProtocol::new(
            "syn".to_string(),
            NetworkProtocol::SYNC.into(),
            &["1".to_string()][..],
            MAX_FRAME_LENGTH_SYNC,
            move || Box::new(synchronizer_clone.clone()),
            Arc::clone(&network_state),
        ),
        CKBProtocol::new(
            "rel".to_string(),
            NetworkProtocol::RELAY.into(),
            &["1".to_string()][..],
            MAX_FRAME_LENGTH_RELAY,
            move || Box::new(relayer.clone()),
            Arc::clone(&network_state),
        ),
        CKBProtocol::new(
            "tim".to_string(),
            NetworkProtocol::TIME.into(),
            &["1".to_string()][..],
            MAX_FRAME_LENGTH_TIME,
            move || Box::new(net_timer.clone()),
            Arc::clone(&network_state),
        ),
        CKBProtocol::new(
            "alt".to_string(),
            NetworkProtocol::ALERT.into(),
            &["1".to_string()][..],
            MAX_FRAME_LENGTH_ALERT,
            move || Box::new(alert_relayer.clone()),
            Arc::clone(&network_state),
        ),
    ];

    let required_protocol_ids = vec![NetworkProtocol::SYNC.into()];

    let network_controller = NetworkService::new(
        Arc::clone(&network_state),
        protocols,
        required_protocol_ids,
        shared.consensus().identify_name(),
        version.to_string(),
        Arc::<(Mutex<()>, Condvar)>::clone(&exit_condvar),
    )
    .start(version, Some("NetworkService"))
    .expect("Start network service failed");

    let builder = ServiceBuilder::new(&args.config.rpc)
        .enable_chain(shared.clone())
        .enable_pool(
            shared.clone(),
            sync_shared,
            args.config.tx_pool.min_fee_rate,
            args.config.rpc.reject_ill_transactions,
        )
        .enable_miner(
            shared.clone(),
            network_controller.clone(),
            chain_controller.clone(),
            miner_enable,
        )
        .enable_net(network_controller.clone())
        .enable_stats(shared.clone(), synchronizer, Arc::clone(&alert_notifier))
        .enable_experiment(shared.clone())
        .enable_integration_test(shared.clone(), network_controller.clone(), chain_controller)
        .enable_alert(alert_verifier, alert_notifier, network_controller)
        .enable_indexer(&args.config.indexer, shared.clone())
        .enable_debug();
    let io_handler = builder.build();

    let rpc_server = RpcServer::new(args.config.rpc, io_handler, shared.notify_controller());

    wait_for_exit(exit_condvar);

    info_target!(crate::LOG_TARGET_MAIN, "Finishing work, please wait...");

    rpc_server.close();
    info_target!(crate::LOG_TARGET_MAIN, "Jsonrpc shutdown");
    Ok(())
}

fn verify_genesis(shared: &Shared) -> Result<(), ExitCode> {
    GenesisVerifier::new()
        .verify(shared.consensus())
        .map_err(|err| {
            eprintln!("genesis error: {}", err);
            ExitCode::Config
        })
}

fn set_system_cell(shared: &Shared) {
    use ckb_error::Error;
    use ckb_store::ChainStore;
    use std::collections::HashMap;

    use ckb_types::{
        core::{
            cell::{resolve_dep_group, CellMeta, CellProvider, ResolvedDep, SYSTEM_CELL},
            DepType,
        },
        packed::{CellDep, OutPoint},
    };

    let genesis = shared.consensus().genesis_block();
    let system_cell_transaction = &genesis.transactions()[0];
    let secp_cell_transaction = &genesis.transactions()[1];
    let secp_code_dep = CellDep::new_builder()
        .out_point(OutPoint::new(system_cell_transaction.hash(), 1))
        .dep_type(DepType::Code.into())
        .build();

    let dao_dep = CellDep::new_builder()
        .out_point(OutPoint::new(system_cell_transaction.hash(), 2))
        .dep_type(DepType::Code.into())
        .build();

    let secp_data_dep = CellDep::new_builder()
        .out_point(OutPoint::new(system_cell_transaction.hash(), 3))
        .dep_type(DepType::Code.into())
        .build();

    let secp_group_dep = CellDep::new_builder()
        .out_point(OutPoint::new(secp_cell_transaction.hash(), 0))
        .dep_type(DepType::DepGroup.into())
        .build();

    let multi_sign_secp_group = CellDep::new_builder()
        .out_point(OutPoint::new(secp_cell_transaction.hash(), 1))
        .dep_type(DepType::DepGroup.into())
        .build();

    let mut cell_deps = HashMap::new();
    let secp_code_dep_cell = shared
        .store()
        .cell_provider()
        .cell_meta(&secp_code_dep.out_point(), true)
        .unwrap()
        .unwrap();
    cell_deps.insert(secp_code_dep, ResolvedDep::Cell(secp_code_dep_cell));

    let dao_dep_cell = shared
        .store()
        .cell_provider()
        .cell_meta(&dao_dep.out_point(), true)
        .unwrap()
        .unwrap();
    cell_deps.insert(dao_dep, ResolvedDep::Cell(dao_dep_cell));

    let secp_data_dep_cell = shared
        .store()
        .cell_provider()
        .cell_meta(&secp_data_dep.out_point(), true)
        .unwrap()
        .unwrap();
    cell_deps.insert(secp_data_dep, ResolvedDep::Cell(secp_data_dep_cell));

    let resolve_cell =
        |out_point: &OutPoint, with_data: bool| -> Result<Option<Box<CellMeta>>, Error> {
            shared
                .store()
                .cell_provider()
                .cell_meta(out_point, with_data)
        };

    let secp_group_dep_cell = resolve_dep_group(&secp_group_dep.out_point(), resolve_cell)
        .unwrap()
        .unwrap();
    cell_deps.insert(secp_group_dep, ResolvedDep::Group(secp_group_dep_cell));

    let multi_sign_secp_group_cell =
        resolve_dep_group(&multi_sign_secp_group.out_point(), resolve_cell)
            .unwrap()
            .unwrap();
    cell_deps.insert(
        multi_sign_secp_group,
        ResolvedDep::Group(multi_sign_secp_group_cell),
    );

    SYSTEM_CELL.set(cell_deps).unwrap();
}

fn sanitize_block_assembler_config(
    args: &RunArgs,
) -> Result<Option<BlockAssemblerConfig>, ExitCode> {
    let block_assembler_config = match (
        args.config.rpc.miner_enable(),
        args.config.block_assembler.clone(),
    ) {
        (true, Some(block_assembler)) => {
            let check_lock_code_hash = |code_hash| -> Result<bool, ExitCode> {
                let secp_cell_data =
                    Resource::bundled("specs/cells/secp256k1_blake160_sighash_all".to_string())
                        .get()
                        .map_err(|err| {
                            eprintln!(
                                "Load specs/cells/secp256k1_blake160_sighash_all error: {:?}",
                                err
                            );
                            ExitCode::Failure
                        })?;
                let genesis_cellbase = &args.consensus.genesis_block().transactions()[0];
                Ok(genesis_cellbase
                    .outputs()
                    .into_iter()
                    .zip(genesis_cellbase.outputs_data().into_iter())
                    .any(|(output, data)| {
                        data.raw_data() == secp_cell_data.as_ref()
                            && output
                                .type_()
                                .to_opt()
                                .map(|script| script.calc_script_hash())
                                .as_ref()
                                == Some(code_hash)
                    }))
            };
            if args.block_assembler_advanced
                || (block_assembler.hash_type == ScriptHashType::Type
                    && block_assembler.args.len() == SECP256K1_BLAKE160_SIGHASH_ALL_ARG_LEN
                    && check_lock_code_hash(&block_assembler.code_hash.pack())?)
            {
                Some(block_assembler)
            } else {
                info_target!(
                    crate::LOG_TARGET_MAIN,
                    "Miner is disabled because block assmebler is not a recommended lock format. \
                     Edit ckb.toml or use `ckb run --ba-advanced` to use other lock scripts"
                );

                None
            }
        }

        _ => {
            info_target!(
                crate::LOG_TARGET_MAIN,
                "Miner is disabled, edit ckb.toml to enable it"
            );

            None
        }
    };
    Ok(block_assembler_config)
}
