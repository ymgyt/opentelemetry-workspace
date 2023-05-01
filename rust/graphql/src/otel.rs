use opentelemetry::Context;
use reqwest::header::{HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Guard to perform opentelemetry termination processing at drop time.
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

#[derive(Debug, thiserror::Error)]
pub enum ContextPropagationError {
    #[error("no context propagated")]
    NoContextPropagated,
    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),
    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}

/// Utility functions to allow client to propagate opentelemetry context.
/// https://opentelemetry.io/docs/reference/specification/context/api-propagators/
pub trait ContextPropagationExt: Sized {
    fn propagate_otel_ctx(self) -> Self {
        let ctx = Span::current().context();
        self.propagate_otel_ctx_with(ctx)
    }

    fn propagate_otel_ctx_with(self, ctx: Context) -> Self {
        match self.try_propagate_otel_ctx_with(ctx) {
            (this, Ok(_)) => this,
            (this, Err(err)) => {
                tracing::warn!(?err, "Failed to propagate opentelemetry context");
                this
            }
        }
    }

    fn try_propagate_otel_ctx_with(
        self,
        ctx: Context,
    ) -> (Self, Result<(), ContextPropagationError>);
}

impl ContextPropagationExt for reqwest::RequestBuilder {
    fn try_propagate_otel_ctx_with(
        self,
        ctx: Context,
    ) -> (Self, Result<(), ContextPropagationError>) {
        let mut injector = PropagationContextInjector {
            builder: Some(self),
            did_inject: false,
            err: None,
        };

        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&ctx, &mut injector)
        });

        let this = injector.builder.unwrap();
        if let Some(err) = injector.err {
            return (this, Err(err));
        }
        if !injector.did_inject {
            (this, Err(ContextPropagationError::NoContextPropagated))
        } else {
            (this, Ok(()))
        }
    }
}

struct PropagationContextInjector {
    builder: Option<reqwest::RequestBuilder>,
    did_inject: bool,
    err: Option<ContextPropagationError>,
}

impl opentelemetry::propagation::Injector for PropagationContextInjector {
    fn set(&mut self, key: &str, value: String) {
        let name = match HeaderName::try_from(key) {
            Ok(name) => name,
            Err(err) => {
                self.err = Some(ContextPropagationError::from(err));
                return;
            }
        };
        let value = match HeaderValue::try_from(value) {
            Ok(value) => value,
            Err(err) => {
                self.err = Some(ContextPropagationError::from(err));
                return;
            }
        };

        self.builder = Some(self.builder.take().unwrap().header(name, value));
        self.did_inject = true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry;
    use reqwest;

    #[tokio::test]
    async fn should_propagate_ctx() {
        #[tracing::instrument()]
        fn instrumented() -> reqwest::RequestBuilder {
            let builder = reqwest::Client::new().get("http://example.com");

            builder.propagate_otel_ctx()
        }

        opentelemetry::global::set_text_map_propagator(
            opentelemetry::sdk::propagation::TraceContextPropagator::new(),
        );

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(opentelemetry_otlp::new_exporter().tonic())
            .with_trace_config(
                opentelemetry::sdk::trace::config()
                    .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                    .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default()),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap();

        use tracing_subscriber::layer::SubscriberExt;
        let subscriber =
            tracing_subscriber::registry().with(tracing_opentelemetry::layer().with_tracer(tracer));

        tracing::subscriber::with_default(subscriber, || {
            let builder = instrumented();

            let req = builder.build().unwrap();
            let header = req.headers();

            // https://w3c.github.io/trace-context/
            assert!(header.get("traceparent").is_some());
            assert!(header.get("tracestate").is_some());
        });
    }
}
