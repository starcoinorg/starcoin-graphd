use starcoin_graphd::graph::DagGraph;
use starcoin_graphd::chain_reader::mock::MockChainReader;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let reader = MockChainReader::new();
    let graph = DagGraph::build(&reader, None, 2).await?;

    let json = serde_json::to_string_pretty(&graph)?;
    std::fs::write("dag.json", json)?;
    Ok(())
}
