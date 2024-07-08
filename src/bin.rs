use std::{fs, net::SocketAddr};
use axum::Extension;

use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sqlx::sqlite::SqlitePoolOptions;

async fn index() -> String {
    String::from("Hello, world!")
}

mod models;
mod controllers;
mod api;
mod errors;

mod timeutil;

// TODO: Shared state struct for Tags,etc wrapped in Arc<RWLock<AppState>>
// Benchmark vs querying individually each time

#[tokio::main]
async fn main() {
    // DB Connection Pooling
    let env = fs::read_to_string(".env").unwrap();
    let (key, db_url) = env.split_once('=').unwrap();

    assert_eq!(key, "DATABASE_URL");

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .unwrap();

    // Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Routing
    let api = axum::Router::new()
        .route("/docs", axum::routing::get(api::doc::get_all_docs))
        .route("/doc/:id", axum::routing::get(api::doc::get_doc))
        .route("/doc", axum::routing::post(api::doc::post_doc))
        .route("/doc", axum::routing::put(api::doc::put_doc));

    let app = axum::Router::new()
        .route("/", axum::routing::get(index))
        .nest_service("/static", ServeDir::new("resources/static"))
        .nest("/api", api)
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
        .fallback(errors::not_found);

    // Network Binding
    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    // App Serving
    axum::serve(listener, app)
        .await
        .unwrap();
}
