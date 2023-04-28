use axum::Extension;
use graphql::handlers;
use graphql::prelude::*;

use axum::Router;
use graphql::schema::QueryRoot;

fn init_subscriber() {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let sampling_ratio = std::env::var("OTEL_SAMPLING_RATIO")
        .ok()
        .and_then(|env| env.parse().ok())
        .unwrap_or(1.0);

    tracing_subscriber::registry::Registry::default()
        .with(fmt::layer().with_ansi(true))
        .with(filter::LevelFilter::INFO)
        .with(tracing_opentelemetry::layer().with_tracer(tracer(sampling_ratio)))
        .init();
}

fn tracer(
    sampling_ratio: f64,
) -> impl opentelemetry::trace::Tracer + tracing_opentelemetry::PreSampledTracer + 'static {
    use opentelemetry::sdk::trace::Sampler;
    use opentelemetry_otlp::WithExportConfig;

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry::sdk::trace::Config::default()
                .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                    sampling_ratio,
                ))))
                .with_id_generator(opentelemetry::sdk::trace::XrayIdGenerator::default())
                .with_resource(opentelemetry::sdk::Resource::new([
                    opentelemetry::KeyValue::new(
                        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                        env!("CARGO_PKG_NAME"),
                    ),
                    opentelemetry::KeyValue::new(
                        opentelemetry_semantic_conventions::resource::SERVICE_VERSION,
                        env!("CARGO_PKG_VERSION"),
                    ),
                ])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .install_batch(opentelemetry::sdk::runtime::Tokio)
        .unwrap()
}

fn app() -> Router {
    use async_graphql::{extensions::Tracing, EmptyMutation, EmptySubscription, Schema};
    use axum::routing::{get, post};
    use tower_http::trace::{DefaultMakeSpan, TraceLayer};

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .finish();

    let middleware = tower::ServiceBuilder::new()
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new()));

    Router::new()
        .route("/health_check", get(handlers::health_check))
        .route("/graphql", post(handlers::graphql))
        .layer(Extension(schema))
        .layer(middleware)
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

    // TODO: call tracer flush
}
