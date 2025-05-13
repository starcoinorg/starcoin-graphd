use async_trait::async_trait;
use starcoin_crypto::HashValue;
use starcoin_types::block::{BlockHeader,BlockNumber};
#[async_trait]
pub trait ChainReader: Send + Sync {
    // get selected chain by blue score number-count to number
    async fn get_selected_chain(
	&self,
	number: Option<BlockNumber>,
        count: u64,
    ) -> anyhow::Result<Vec<BlockHeader>>;
    
    async fn get_ghostdag_data(&self, hash:HashValue) ->  anyhow::Result<Option<GhostdagData>>;
    async fn get_headers(&self, block_hashes: Vec<HashValue>) -> anyhow::Result<Vec<BlockHeader>>;
}

#[derive(Clone)]
pub struct GhostdagData {
    pub blue_score: u64,
    pub selected_parent: HashValue,
    pub mergeset_blues: Vec<HashValue>,
    pub mergeset_reds: Vec<HashValue>,
}


pub mod mock;
