use graphql::handlers;
use graphql::prelude::*;

use axum::Router;

fn init_subscriber() {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry::Registry::default()
        .with(fmt::layer().with_ansi(true))
        .with(filter::LevelFilter::INFO)
        .init();
}

fn app() -> Router {
    use axum::routing::get;

    Router::new().route("/health_check", get(handlers::health_check))
}

#[tokio::main]
async fn main() {
    init_subscriber();

    let app = app();

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));

    info!(%addr, "Listening...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
