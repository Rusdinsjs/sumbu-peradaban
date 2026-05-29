mod db;
mod error;
mod graphql;
mod models;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::graphql::build_schema;

/// Application state holding the GraphQL schema.
type AppSchema = async_graphql::Schema<
    graphql::query::QueryRoot,
    graphql::mutation::MutationRoot,
    async_graphql::EmptySubscription,
>;

/// Handler for POST /graphql — executes a GraphQL request.
async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Handler for GET / — serves the GraphiQL interactive playground.
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env (silently ignore if missing)
    dotenvy::dotenv().ok();

    // Initialize tracing subscriber with env-filter support (e.g. RUST_LOG=info)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("Starting Sumbu Peradaban backend…");

    // --- PostgreSQL ---
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::postgres::init_pool(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Run SQLx migrations embedded at compile time
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    info!("Database migrations applied successfully");

    // --- Neo4j ---
    let neo4j_uri =
        std::env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
    let neo4j_user =
        std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let neo4j_pass =
        std::env::var("NEO4J_PASS").expect("NEO4J_PASS must be set");

    let graph = db::neo4j::init_graph(&neo4j_uri, &neo4j_user, &neo4j_pass)
        .await
        .expect("Failed to connect to Neo4j");

    // --- Build GraphQL Schema ---
    let schema = build_schema(pool, graph);
    info!("GraphQL schema built");

    // --- CORS ---
    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());

    // --- Axum Router ---
    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .with_state(schema)
        .layer(cors);

    // --- Start Server ---
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to 0.0.0.0:8080");
    info!("Server listening on http://0.0.0.0:8080");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
