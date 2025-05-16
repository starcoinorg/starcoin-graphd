use starcoin_graphd::chain_reader::mock::MockChainReader;
use starcoin_graphd::chain_reader::BlockWindow;
use starcoin_graphd::dag_graph::DagGraph;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let reader = MockChainReader::new();
    let graph = DagGraph::build(&reader, BlockWindow::Latest(2)).await?;

    let json = serde_json::to_string_pretty(&graph)?;
    std::fs::write("dag.json", json)?;
    Ok(())
}
