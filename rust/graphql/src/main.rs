use axum::Extension;
use axum::Router;
use graphql::trace_layer;
use graphql::{client, handlers, otel, prelude::*, schema::QueryRoot};

fn init_subscriber() {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let sampling_ratio = std::env::var("OTEL_SAMPLING_RATIO")
        .ok()
        .and_then(|env| env.parse().ok())
        .unwrap_or(1.0);

    let metrics_layer = tracing_opentelemetry::MetricsLayer::new(meter_provider());

    tracing_subscriber::registry::Registry::default()
        .with(fmt::layer().with_ansi(true))
        .with(filter::LevelFilter::INFO)
        .with(tracing_opentelemetry::layer().with_tracer(tracer(sampling_ratio)))
        .with(metrics_layer)
        .init();
}

fn meter_provider() -> opentelemetry::sdk::metrics::MeterProvider {
    use opentelemetry_otlp::WithExportConfig;

    opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry::runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_period(std::time::Duration::from_secs(5))
        .build()
        .unwrap()
}

fn tracer(sampling_ratio: f64) -> opentelemetry::sdk::trace::Tracer {
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
                .with_resource(opentelemetry::sdk::Resource::from_schema_url(
                    [
                        opentelemetry::KeyValue::new(
                            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                            env!("CARGO_PKG_NAME"),
                        ),
                        opentelemetry::KeyValue::new(
                            opentelemetry_semantic_conventions::resource::SERVICE_VERSION,
                            env!("CARGO_PKG_VERSION"),
                        ),
                    ],
                    "https://opentelemetry.io/schemas/1.20.0",
                )),
        )
        .with_batch_config(opentelemetry::sdk::trace::BatchConfig::default())
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
    use http::{
        header::{self, HeaderName},
        Method,
    };
    use tower_http::cors::{self, CorsLayer};

    let rest_client = client::RestClient::new();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .data(rest_client)
        .finish();

    // ServiceBuilder top to bottom
    // https://docs.rs/axum/latest/axum/middleware/index.html
    let middleware = tower::ServiceBuilder::new()
        .layer(trace_layer::trace_layer())
        .layer(
            CorsLayer::new()
                .allow_headers([
                    header::CONTENT_TYPE,
                    // W3C context trace propagator headers
                    HeaderName::from_static("traceparent"),
                    HeaderName::from_static("tracestate"),
                ])
                .allow_origin(cors::Any)
                .allow_methods([Method::GET, Method::POST]),
        );

    Router::new()
        .route("/health_check", get(handlers::health_check))
        .route("/graphql", post(handlers::graphql))
        .layer(Extension(schema))
        .layer(middleware)
}

#[tokio::main]
async fn main() {
    let _guard = otel::init_opentelemetry();
    init_subscriber();

    foo();

    // MeterProvider::shutdown() is not implemented yet in current version(0.19)

    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    /*
    let app = app();

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));

    info!(%addr, "Listening...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // TODO: call tracer flush
    */
}

#[tracing::instrument]
fn foo() {
    info!(monotonic_counter.foo = 1, key_1 = "value_1", "foo",);
}
