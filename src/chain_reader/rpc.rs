use crate::chain_reader::ext::{BlockHeader, ChainReaderExt, GhostdagData};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use starcoin_crypto::HashValue;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct RpcChainReader {
    rpc_url: String,
    client: reqwest::Client,
    id_counter: AtomicU64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcGhostdagData {
    pub blue_score: u64,
    pub blue_work: String, //u256 string
    pub blues_anticone_sizes: HashMap<HashValue, u64>,
    pub mergeset_blues: Vec<HashValue>,
    pub mergeset_reds: Vec<HashValue>,
    pub selected_parent: HashValue,
}

impl TryFrom<RpcGhostdagData> for GhostdagData {
    type Error = anyhow::Error;

    fn try_from(rpc: RpcGhostdagData) -> Result<Self> {
        Ok(Self {
            blue_score: rpc.blue_score,
            selected_parent: rpc.selected_parent,
            mergeset_blues: rpc.mergeset_blues,
            mergeset_reds: rpc.mergeset_reds,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcBlockHeader {
    pub block_hash: String,
    pub body_hash: String,
    pub chain_id: u8,
    pub difficulty: String,
    pub extra: String,
    pub gas_used: String,
    pub nonce: u64,
    pub number: String,
    pub parent_hash: String,
    pub parents_hash: Vec<String>,
    pub pruning_point: String,
    pub timestamp: String,
    pub version: u32,
}

impl TryFrom<RpcBlockHeader> for BlockHeader {
    type Error = anyhow::Error;

    fn try_from(rpc: RpcBlockHeader) -> Result<Self> {
        let parents_hash = rpc
            .parents_hash
            .iter()
            .map(|s| HashValue::from_hex_literal(s))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BlockHeader {
            id: HashValue::from_hex_literal(&rpc.block_hash)?,
            parents_hash,
            number: rpc.number.parse::<u64>()?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcBlock {
    pub header: RpcBlockHeader,
}

impl RpcChainReader {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            client: reqwest::Client::new(),
            id_counter: AtomicU64::new(1),
        }
    }

    async fn rpc_call<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> anyhow::Result<T> {
        let id = self.id_counter.fetch_add(1, Ordering::Relaxed);
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let resp = self
            .client
            .post(&self.rpc_url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        let result = resp
            .get("result")
            .ok_or_else(|| anyhow::anyhow!("Missing 'result' field"))?;
        Ok(serde_json::from_value(result.clone())?)
    }
}

#[async_trait::async_trait]
impl ChainReaderExt for RpcChainReader {
    async fn get_ghostdag_data(
        &self,
        ids: &[HashValue],
    ) -> anyhow::Result<Vec<Option<GhostdagData>>> {
        let id_hexes: Vec<String> = ids.iter().map(|h| h.to_hex_literal()).collect();
        let params = serde_json::json!([id_hexes]);
        let rpc_result: Vec<Option<RpcGhostdagData>> =
            self.rpc_call("chain.get_ghostdagdata", params).await?;
        let converted = rpc_result
            .into_iter()
            .map(|opt| opt.map(|r| r.try_into()).transpose())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(converted)
    }

    async fn get_selected_chain(
        &self,
        number: Option<u64>,
        count: u64,
    ) -> Result<Vec<BlockHeader>> {
        let params = serde_json::json!([number, count]);
        let rpc_blocks: Vec<RpcBlock> = self.rpc_call("chain.get_blocks_by_number", params).await?;
        let headers = rpc_blocks
            .into_iter()
            .map(|b| b.header.try_into()) // RpcBlockHeader → BlockHeader
            .collect::<Result<Vec<_>, _>>()?;

        Ok(headers)
    }
    async fn get_headers(&self, ids: &[HashValue]) -> anyhow::Result<Vec<BlockHeader>> {
        let id_hexes: Vec<String> = ids.iter().map(|h| h.to_hex_literal()).collect();
        let params = serde_json::json!([id_hexes]);
        let rpc_result: Vec<RpcBlockHeader> = self.rpc_call("chain.get_headers", params).await?;
        let headers = rpc_result
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(headers)
    }
}

#[tokio::test]
async fn test_get_ghostdag_data() -> Result<()> {
    let reader = RpcChainReader::new("http://localhost:32799");
    let hash = "0x2aec8e48fb6d8c52ce09833f63146a822ee01c0ed58d42794f2c02d49cbfa66e";
    let id = HashValue::from_hex_literal(hash)?;
    let result = reader.get_ghostdag_data(&[id]).await?;
    assert_eq!(result.len(), 1);
    let data = result[0].as_ref().expect("Should have data");
    println!("{:#?}", data);
    Ok(())
}

#[tokio::test]
async fn test_get_headers() -> Result<()> {
    let reader = RpcChainReader::new("http://localhost:32799");
    let hash = "0x2aec8e48fb6d8c52ce09833f63146a822ee01c0ed58d42794f2c02d49cbfa66e";
    let id = HashValue::from_hex_literal(hash)?;
    let result = reader.get_headers(&[id]).await?;
    assert_eq!(result.len(), 1);
    println!("{:?}", result);
    Ok(())
}

#[tokio::test]
async fn test_get_selected_chain() -> Result<()> {
    let reader = RpcChainReader::new("http://localhost:32799");
    let result = reader.get_selected_chain(Some(3), 2).await?;
    assert_eq!(result.len(), 2);
    println!("{:?}", result);
    Ok(())
}
