use deadpool_lapin::lapin::{
    options::BasicPublishOptions,
    publisher_confirm::Confirmation,
    types::{AMQPValue, FieldTable, LongString, ShortString},
    BasicProperties, Channel,
};
use queue;
use tracing_opentelemetry::OpenTelemetrySpanExt;

const QUEUE: &str = "example";

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
    let channel: Channel = queue::channel().await?;

    let queue = channel
        .queue_declare(
            QUEUE,
            deadpool_lapin::lapin::options::QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tracing::info!(?queue, "Start producer");

    let mut counter = 0;
    loop {
        let span = tracing::info_span!("producer").entered();

        tracing::info!(%counter,"producing...");

        counter += 1;
        let payload = Vec::from(format!("Hello {counter}"));

        // Inject opentelemetry context
        let mut table = {
            let ctx = tracing::Span::current().context();
            let mut table = FieldTable::default();

            opentelemetry::global::get_text_map_propagator(|propagator| {
                propagator.inject_context(
                    &ctx,
                    &mut FieldTableInjector {
                        field_table: &mut table,
                    },
                )
            });

            table
        };

        let properties = BasicProperties::default();
        let properties = properties.with_headers(table);

        let confirm = channel
            .basic_publish(
                "",
                QUEUE,
                BasicPublishOptions::default(),
                &payload,
                properties,
            )
            .await?
            .await?;

        assert_eq!(confirm, Confirmation::NotRequested);

        drop(span);

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

pub struct FieldTableInjector<'a> {
    pub field_table: &'a mut FieldTable,
}

impl<'a> opentelemetry::propagation::Injector for FieldTableInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.field_table.insert(
            ShortString::from(key),
            AMQPValue::LongString(LongString::from(value)),
        );
    }
}
