pub mod query;
pub mod mutation;

use async_graphql::{Schema, EmptySubscription};
use sqlx::PgPool;
use neo4rs::Graph;

pub type AppSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;

/// Builds the async-graphql schema and injects database pools as shared context data.
pub fn build_schema(pool: PgPool, graph: Graph) -> AppSchema {
    Schema::build(
        query::QueryRoot::default(),
        mutation::MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .data(graph)
    .finish()
}
