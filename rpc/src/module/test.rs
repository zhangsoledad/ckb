use crate::error::RPCError;
use ckb_chain::chain::ChainController;
use ckb_jsonrpc_types::{Block, Transaction};
use ckb_logger::error;
use ckb_network::NetworkController;
use ckb_shared::shared::Shared;
use ckb_sync::NetworkProtocol;
use ckb_types::{core, packed, prelude::*, H256};
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use std::sync::Arc;

#[rpc]
pub trait IntegrationTestRpc {
    // curl -d '{"id": 2, "jsonrpc": "2.0", "method":"add_node","params": ["QmUsZHPbjjzU627UZFt4k8j6ycEcNvXRnVGxCPKqwbAfQS", "/ip4/192.168.2.100/tcp/30002"]}' -H 'content-type:application/json' 'http://localhost:8114'
    #[rpc(name = "add_node")]
    fn add_node(&self, peer_id: String, address: String) -> Result<()>;

    // curl -d '{"id": 2, "jsonrpc": "2.0", "method":"remove_node","params": ["QmUsZHPbjjzU627UZFt4k8j6ycEcNvXRnVGxCPKqwbAfQS"]}' -H 'content-type:application/json' 'http://localhost:8114'
    #[rpc(name = "remove_node")]
    fn remove_node(&self, peer_id: String) -> Result<()>;

    #[rpc(name = "process_block_without_verify")]
    fn process_block_without_verify(&self, data: Block) -> Result<Option<H256>>;

    #[rpc(name = "broadcast_transaction")]
    fn broadcast_transaction(&self, transaction: Transaction) -> Result<H256>;
}

pub(crate) struct IntegrationTestRpcImpl {
    pub network_controller: NetworkController,
    pub shared: Shared,
    pub chain: ChainController,
}

impl IntegrationTestRpc for IntegrationTestRpcImpl {
    fn add_node(&self, peer_id: String, address: String) -> Result<()> {
        self.network_controller.add_node(
            &peer_id.parse().expect("invalid peer_id"),
            address.parse().expect("invalid address"),
        );
        Ok(())
    }

    fn remove_node(&self, peer_id: String) -> Result<()> {
        self.network_controller
            .remove_node(&peer_id.parse().expect("invalid peer_id"));
        Ok(())
    }

    fn process_block_without_verify(&self, data: Block) -> Result<Option<H256>> {
        let block: packed::Block = data.into();
        let block: Arc<core::BlockView> = Arc::new(block.to_view());
        let ret = self.chain.process_block(Arc::clone(&block), false);
        if ret.is_ok() {
            Ok(Some(block.hash().unpack()))
        } else {
            error!("process_block_without_verify error: {:?}", ret);
            Ok(None)
        }
    }

    fn broadcast_transaction(&self, transaction: Transaction) -> Result<H256> {
        let tx: packed::Transaction = transaction.into();
        let hash = tx.calc_tx_hash();
        let relay_tx = packed::RelayTransaction::new_builder()
            .cycles(10000u64.pack())
            .transaction(tx)
            .build();
        let relay_txs = packed::RelayTransactions::new_builder()
            .transactions(vec![relay_tx].pack())
            .build();
        let message = packed::RelayMessage::new_builder().set(relay_txs).build();
        let data = message.as_slice().into();
        if let Err(err) = self
            .network_controller
            .broadcast(NetworkProtocol::RELAY.into(), data)
        {
            error!("Broadcast transaction failed: {:?}", err);
            Err(RPCError::custom(RPCError::Invalid, err.to_string()))
        } else {
            Ok(hash)
        }
    }
}
