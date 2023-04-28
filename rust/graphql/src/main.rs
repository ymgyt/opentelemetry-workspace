use axum::Extension;
use graphql::handlers;
use graphql::prelude::*;

use axum::Router;
use graphql::schema::QueryRoot;

fn init_subscriber() {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry::Registry::default()
        .with(fmt::layer().with_ansi(true))
        .with(filter::LevelFilter::INFO)
        .with(tracing_opentelemetry::layer().with_tracer(
            opentelemetry::sdk::export::trace::stdout::new_pipeline().install_simple(),
        ))
        .init();
}

fn app() -> Router {
    use async_graphql::{extensions::Tracing, EmptyMutation, EmptySubscription, Schema};
    use axum::routing::{get, post};

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .finish();

    Router::new()
        .route("/health_check", get(handlers::health_check))
        .route("/graphql", post(handlers::graphql))
        .layer(Extension(schema))
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
