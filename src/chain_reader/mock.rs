use crate::chain_reader::ext::{BlockHeader, GhostdagData};
use async_trait::async_trait;
use starcoin_crypto::HashValue;
use std::collections::HashMap;

use super::ext::ChainReaderExt;
pub struct MockChainReader {
    selected_chain: Vec<BlockHeader>,
    ghostdag_map: HashMap<HashValue, GhostdagData>,
    header_map: HashMap<HashValue, BlockHeader>,
}

impl MockChainReader {
    pub fn new() -> Self {
        // genesis block A
        let block_a = BlockHeader {
            id: HashValue::random(),
            number: 0,
            parents_hash: vec![],
        };

        // block B
        let block_b = BlockHeader {
            id: HashValue::random(),
            number: 1,
            parents_hash: vec![block_a.id],
        };

        // block C
        let block_c = BlockHeader {
            id: HashValue::random(),
            number: 2,
            parents_hash: vec![block_b.id],
        };

        // block D
        let block_d = BlockHeader {
            id: HashValue::random(),
            number: 3,
            parents_hash: vec![block_b.id],
        };

        // block E
        let block_e = BlockHeader {
            id: HashValue::random(),
            number: 4,
            parents_hash: vec![block_c.id, block_d.id],
        };

        // block F
        let block_f = BlockHeader {
            id: HashValue::random(),
            number: 5,
            parents_hash: vec![block_d.id],
        };

        // block G
        let block_g = BlockHeader {
            id: HashValue::random(),
            number: 6,
            parents_hash: vec![block_d.id],
        };

        // block H
        let block_h = BlockHeader {
            id: HashValue::random(),
            number: 7,
            parents_hash: vec![block_e.id, block_f.id, block_g.id],
        };

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
impl ChainReaderExt for MockChainReader {
    async fn get_selected_chain(
        &self,
        _number: Option<u64>,
        _count: u64,
    ) -> anyhow::Result<Vec<BlockHeader>> {
        Ok(self.selected_chain.clone())
    }

    async fn get_ghostdag_data(
        &self,
        ids: &[HashValue],
    ) -> anyhow::Result<Vec<Option<GhostdagData>>> {
        let ret: Vec<Option<GhostdagData>> = ids
            .iter()
            .map(|id| self.ghostdag_map.get(id).cloned())
            .collect();
        Ok(ret)
    }

    async fn get_headers(&self, ids: &[HashValue]) -> anyhow::Result<Vec<BlockHeader>> {
        let ret = ids
            .iter()
            .filter_map(|id| self.header_map.get(id).cloned())
            .collect();
        Ok(ret)
    }
}
