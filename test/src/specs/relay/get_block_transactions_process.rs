use crate::{Net, Spec, TestProtocol};
use ckb_sync::NetworkProtocol;
use ckb_types::{
    core::UncleBlockView,
    packed::{self, RelayMessage},
    prelude::*,
};

pub struct MissingUncleRequest;

impl Spec for MissingUncleRequest {
    crate::name!("missing_uncle_request");

    crate::setup!(protocols: vec![TestProtocol::sync(), TestProtocol::relay()]);

    // Case: Send to node GetBlockTransactions with missing uncle index, node should response BlockTransactions with uncles
    fn run(&self, net: Net) {
        net.exit_ibd_mode();
        let node = &net.nodes[0];
        net.connect(node);
        let (peer_id, _, _) = net.receive();

        node.generate_block();
        let _ = net.receive();

        let builder = node.new_block_builder(None, None, None);
        let block1 = builder.clone().nonce(0.pack()).build();
        let block2 = builder.clone().nonce(1.pack()).build();
        node.submit_block(&block1.data());
        node.submit_block(&block2.data());

        let builder = node.new_block_builder(None, None, None);
        let block = builder
            .clone()
            .set_uncles(vec![block2.as_uncle()])
            .nonce(0.pack())
            .build();
        node.submit_block(&block.data());

        let content = packed::GetBlockTransactions::new_builder()
            .block_hash(block.hash())
            .uncle_indexes(vec![0u32].pack())
            .build();
        let message = packed::RelayMessage::new_builder().set(content).build();

        (0..3).for_each(|_| {
            net.receive(); // ignore three new block announce
        });

        net.send(
            NetworkProtocol::RELAY.into(),
            peer_id,
            message.as_slice().into(),
        );

        let (_, _, data) = net.receive();
        let message = RelayMessage::from_slice(&data).unwrap();

        assert_eq!(
            message.to_enum().item_name(),
            packed::BlockTransactions::NAME,
            "Node should reponse BlockTransactions message",
        );

        if let packed::RelayMessageUnionReader::BlockTransactions(reader) =
            message.to_enum().as_reader()
        {
            let block_transactions = reader.to_entity();
            let received_uncles: Vec<UncleBlockView> = block_transactions
                .uncles()
                .into_iter()
                .map(|uncle| uncle.into_view())
                .collect();
            assert_eq!(received_uncles[0], block2.as_uncle());
        }
    }
}
