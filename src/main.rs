use axum::{Router, routing::get, routing::post};

mod config;
mod handlers;

#[tokio::main]
async fn main() {
    let config = config::Config::load();
    let addr = format!("{}:{}", config.server_host, config.server_port);

    let app = Router::new()
        .route("/", get(|| async { "Smart on FHIR Token Broker" }))
        .route("/health", get(handlers::index::get_health))
        .route("/token/validate", get(handlers::index::get_token_validate))
        .route(
            "/token/exchange",
            post(handlers::index::post_token_exchange),
        );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
