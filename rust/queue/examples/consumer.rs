use deadpool_lapin::lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    publisher_confirm::Confirmation,
    types::{AMQPValue, FieldTable, LongString, ShortString},
    BasicProperties,
};
use queue::{self, Channel};

#[must_use]
pub struct OtelInitGuard();

impl Drop for OtelInitGuard {
    fn drop(&mut self) {
        opentelemetry::global::shutdown_tracer_provider();
    }
}

pub fn init_opentelemetry() -> OtelInitGuard {
    // for context propagation.
    opentelemetry::global::set_text_map_propagator(
        opentelemetry::sdk::propagation::TraceContextPropagator::new(),
    );

    OtelInitGuard()
}

const QUEUE: &str = "example";
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
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .install_batch(opentelemetry::sdk::runtime::Tokio)
        .unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    let _guard = init_opentelemetry();

    let channel = queue::channel().await?;

    let queue = channel
        .queue_declare(
            QUEUE,
            deadpool_lapin::lapin::options::QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tracing::info!(?queue, "Start consumer");

    let mut consumer = channel
        .basic_consume(
            QUEUE,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    use futures::stream::StreamExt;
    while let Some(delivery) = consumer.next().await.transpose()? {
        let span = tracing::info_span!("consumer").entered();

        let message = String::from_utf8_lossy(delivery.data.as_slice());

        // Extract opentelemetry context
        if let Some(field_table) = delivery.properties.headers().as_ref() {
            let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
                propagator.extract(&FieldTableExtractor {
                    field_table: &field_table,
                })
            });

            use tracing_opentelemetry::OpenTelemetrySpanExt;

            span.set_parent(parent_context);
        }

        tracing::info!(?message, "Received");

        delivery.ack(BasicAckOptions::default()).await?;

        drop(span);
    }
    Ok(())
}

pub struct FieldTableExtractor<'a> {
    pub field_table: &'a FieldTable,
}

impl<'a> opentelemetry::propagation::Extractor for FieldTableExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.field_table
            .inner()
            .get(key)
            .and_then(|amqp_value| amqp_value.as_long_string())
            .map(|long_string| long_string.as_bytes())
            .and_then(|bytes| {
                std::str::from_utf8(bytes)
                    .map_err(|err| {
                        tracing::warn!(?err, "Opentelemetry context contains invalid utf8");
                        err
                    })
                    .ok()
            })
    }

    fn keys(&self) -> Vec<&str> {
        self.field_table
            .inner()
            .keys()
            .map(|short_string| short_string.as_str())
            .collect()
    }
}
