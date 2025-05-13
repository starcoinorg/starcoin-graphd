use crate::chain_reader::{ChainReader, GhostdagData};
use async_trait::async_trait;
use starcoin_crypto::HashValue;
use starcoin_types::block::{BlockHeader, BlockHeaderBuilder, BlockNumber};
use std::collections::HashMap;
pub struct MockChainReader {
    selected_chain: Vec<BlockHeader>,
    ghostdag_map: HashMap<HashValue, GhostdagData>,
    header_map: HashMap<HashValue, BlockHeader>,
}

impl MockChainReader {
    pub fn new() -> Self {
        // Construct a simple DAG:
        // A (genesis)
        //  \
        //   B (selected, parent: A)
        //   |\
        //   | C (merge blue, parent: A)
        //   | D (merge red, parent: A)
        //
        // E (selected, parent: B)

        let block_a = BlockHeader::dag_genesis_random(0);
        let block_b = BlockHeaderBuilder::random()
            .with_number(1)
            .with_parents_hash(vec![block_a.id()])
            .build();
        let block_c = BlockHeaderBuilder::random()
            .with_number(2)
            .with_parents_hash(vec![block_b.id()])
            .build();
        let block_d = BlockHeaderBuilder::random()
            .with_number(3)
            .with_parents_hash(vec![block_b.id()])
            .build();
        let block_e = BlockHeaderBuilder::random()
            .with_number(4)
            .with_parents_hash(vec![block_c.id(), block_d.id()])
            .build();
        let block_f = BlockHeaderBuilder::random()
            .with_number(5)
            .with_parents_hash(vec![block_d.id()])
            .build();
        let block_g = BlockHeaderBuilder::random()
            .with_number(6)
            .with_parents_hash(vec![block_d.id()])
            .build();
        let block_h = BlockHeaderBuilder::random()
            .with_number(6)
            .with_parents_hash(vec![block_e.id(), block_f.id(), block_g.id()])
            .build();

        let mut ghostdag_map = HashMap::new();
        let ghostdag_b = GhostdagData {
            blue_score: 1,
            selected_parent: block_a.id(),
            mergeset_blues: vec![block_a.id()],
            mergeset_reds: vec![],
        };
        let ghostdag_c = GhostdagData {
            blue_score: 2,
            selected_parent: block_b.id(),
            mergeset_blues: vec![block_b.id()],
            mergeset_reds: vec![],
        };
        let ghostdag_d = GhostdagData {
            blue_score: 3,
            selected_parent: block_b.id(),
            mergeset_blues: vec![block_b.id()],
            mergeset_reds: vec![],
        };
        let ghostdag_e = GhostdagData {
            blue_score: 4,
            selected_parent: block_c.id(),
            mergeset_blues: vec![block_c.id(), block_d.id()],
            mergeset_reds: vec![],
        };
        let ghostdag_f = GhostdagData {
            blue_score: 5,
            selected_parent: block_d.id(),
            mergeset_blues: vec![block_d.id()],
            mergeset_reds: vec![],
        };
        let ghostdag_g = GhostdagData {
            blue_score: 5,
            selected_parent: block_d.id(),
            mergeset_blues: vec![block_d.id()],
            mergeset_reds: vec![],
        };
        let ghostdag_h = GhostdagData {
            blue_score: 6,
            selected_parent: block_e.id(),
            mergeset_blues: vec![block_e.id(), block_f.id()],
            mergeset_reds: vec![block_g.id()],
        };

        ghostdag_map.insert(block_b.id(), ghostdag_b);
        ghostdag_map.insert(block_c.id(), ghostdag_c);
        ghostdag_map.insert(block_d.id(), ghostdag_d);
        ghostdag_map.insert(block_e.id(), ghostdag_e);
        ghostdag_map.insert(block_f.id(), ghostdag_f);
        ghostdag_map.insert(block_g.id(), ghostdag_g);
        ghostdag_map.insert(block_h.id(), ghostdag_h);

        let mut header_map = HashMap::new();
        header_map.insert(block_a.id(), block_a.clone());
        header_map.insert(block_b.id(), block_b.clone());
        header_map.insert(block_c.id(), block_c.clone());
        header_map.insert(block_d.id(), block_d.clone());
        header_map.insert(block_e.id(), block_e.clone());
	header_map.insert(block_f.id(), block_f.clone());
	header_map.insert(block_g.id(), block_g.clone());
	header_map.insert(block_h.id(), block_h.clone());
        let selected_chain = vec![block_b, block_c, block_e, block_h];

        Self {
            selected_chain,
            ghostdag_map,
            header_map,
        }
    }
}

#[async_trait]
impl ChainReader for MockChainReader {
    async fn get_selected_chain(
        &self,
        _number: Option<BlockNumber>,
        _count: u64,
    ) -> anyhow::Result<Vec<BlockHeader>> {
        Ok(self.selected_chain.clone())
    }

    async fn get_ghostdag_data(&self, hash: HashValue) -> anyhow::Result<Option<GhostdagData>> {
        Ok(self.ghostdag_map.get(&hash).cloned())
    }

    async fn get_headers(&self, block_hashes: Vec<HashValue>) -> anyhow::Result<Vec<BlockHeader>> {
        let result = block_hashes
            .into_iter()
            .filter_map(|id| self.header_map.get(&id).cloned())
            .collect();
        Ok(result)
    }
}
