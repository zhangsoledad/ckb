use ckb_app_config::TxPoolConfig;
use ckb_chain::chain::{ChainController, ChainService};
use ckb_chain_spec::consensus::ConsensusBuilder;
use ckb_network;
use ckb_network::{Behaviour, CKBProtocolContext, Peer, PeerIndex, ProtocolId, TargetSession};
use ckb_pow::Pow;
use ckb_shared::shared::{Shared, SharedBuilder};
use ckb_sync::synchronizer::headers_process::HeadersProcess;
use ckb_sync::{SyncShared, Synchronizer};
use ckb_types::{
    bytes::Bytes,
    core::{FeeRate, HeaderBuilder, HeaderView},
    packed::SendHeadersBuilder,
    prelude::*,
};
use criterion::{criterion_group, BatchSize, Criterion};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

pub fn setup() -> (Vec<HeaderView>, Shared, ChainController) {
    let mut consensus = ConsensusBuilder::default().build();
    consensus.permanent_difficulty_in_dummy = true;
    consensus.pow = Pow::Dummy;
    let mut tx_pool_config = TxPoolConfig::default();
    tx_pool_config.min_fee_rate = FeeRate::from_u64(0);

    let (shared, table) = SharedBuilder::with_temp_db()
        .consensus(consensus.clone())
        .tx_pool_config(tx_pool_config.clone())
        .build()
        .unwrap();
    let chain_service = ChainService::new(shared.clone(), table);
    let chain_controller = chain_service.start(Some("ChainService"));

    let mut headers = Vec::with_capacity(2000);
    let mut parent = shared.snapshot().tip_header().clone();

    for _ in 0..2000 {
        let header = gen_header(&parent);
        headers.push(header.clone());
        parent = header;
    }

    (headers, shared, chain_controller)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn gen_header(parent: &HeaderView) -> HeaderView {
    let number = parent.number() + 1;

    HeaderBuilder::default()
        .parent_hash(parent.hash())
        .timestamp((parent.timestamp() + 10).pack())
        .number(number.pack())
        .build()
}

#[derive(Default)]
pub(crate) struct DummyNetworkContext;

impl CKBProtocolContext for DummyNetworkContext {
    fn set_notify(&self, _interval: Duration, _token: u64) -> Result<(), ckb_network::Error> {
        unimplemented!();
    }

    fn remove_notify(&self, _token: u64) -> Result<(), ckb_network::Error> {
        unimplemented!()
    }

    fn future_task(
        &self,
        _task: Pin<Box<dyn Future<Output = ()> + 'static + Send>>,
        _blocking: bool,
    ) -> Result<(), ckb_network::Error> {
        //            task.await.expect("resolve future task error");
        Ok(())
    }

    fn quick_send_message(
        &self,
        proto_id: ProtocolId,
        peer_index: PeerIndex,
        data: Bytes,
    ) -> Result<(), ckb_network::Error> {
        self.send_message(proto_id, peer_index, data)
    }
    fn quick_send_message_to(
        &self,
        peer_index: PeerIndex,
        data: Bytes,
    ) -> Result<(), ckb_network::Error> {
        self.send_message_to(peer_index, data)
    }
    fn quick_filter_broadcast(
        &self,
        target: TargetSession,
        data: Bytes,
    ) -> Result<(), ckb_network::Error> {
        self.filter_broadcast(target, data)
    }
    fn send_message(
        &self,
        _proto_id: ProtocolId,
        _peer_index: PeerIndex,
        _data: Bytes,
    ) -> Result<(), ckb_network::Error> {
        Ok(())
    }
    fn send_message_to(
        &self,
        _peer_index: PeerIndex,
        _data: Bytes,
    ) -> Result<(), ckb_network::Error> {
        Ok(())
    }
    fn filter_broadcast(
        &self,
        _target: TargetSession,
        _data: Bytes,
    ) -> Result<(), ckb_network::Error> {
        Ok(())
    }
    fn disconnect(&self, _peer_index: PeerIndex, _msg: &str) -> Result<(), ckb_network::Error> {
        Ok(())
    }
    // Interact with NetworkState
    fn get_peer(&self, _peer_index: PeerIndex) -> Option<Peer> {
        None
    }
    fn with_peer_mut(&self, _peer_index: PeerIndex, _f: Box<dyn FnOnce(&mut Peer)>) {}
    fn connected_peers(&self) -> Vec<PeerIndex> {
        unimplemented!();
    }
    fn report_peer(&self, _peer_index: PeerIndex, _behaviour: Behaviour) {}
    fn ban_peer(&self, _peer_index: PeerIndex, _duration: Duration, _reason: String) {}
    // Other methods
    fn protocol_id(&self) -> ProtocolId {
        unimplemented!();
    }
}

fn gen_synchronizer(chain_controller: ChainController, shared: Shared) -> Synchronizer {
    let shared = Arc::new(SyncShared::new(shared, Default::default()));
    Synchronizer::new(chain_controller, shared)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("headers_process");
    group.bench_function("headers_process/2000", |b| {
        b.iter_batched(
            || setup(),
            |(headers, shared, chain)| {
                let syncer = gen_synchronizer(chain, shared);

                let sendheaders = SendHeadersBuilder::default()
                    .headers(headers.iter().map(|h| h.data()).pack())
                    .build();
                let peer: PeerIndex = 1.into();

                let dummy_nc = DummyNetworkContext::default();
                HeadersProcess::new(sendheaders.as_reader(), &syncer, peer, &dummy_nc).execute()
            },
            BatchSize::PerIteration,
        )
    });
}

criterion_group!(
    name = headers_process;
    config = Criterion::default().sample_size(10);
    targets = bench
);
