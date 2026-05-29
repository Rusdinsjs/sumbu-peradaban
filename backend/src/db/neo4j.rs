use neo4rs::Graph;
use tracing::info;

pub async fn init_graph(uri: &str, user: &str, pass: &str) -> Result<Graph, neo4rs::Error> {
    info!("Connecting to Neo4j at {}...", uri);
    let graph = Graph::new(uri, user, pass).await?;
    info!("Neo4j connected successfully");
    Ok(graph)
}
