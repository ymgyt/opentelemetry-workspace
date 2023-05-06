use http::{header::HeaderName, request::Request};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultOnRequest, DefaultOnResponse, MakeSpan},
};
use tracing::{span::Span, Level};

pub type TraceLayer = tower_http::trace::TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    MakeSpanImpl,
    DefaultOnRequest,
    DefaultOnResponse,
>;

pub fn trace_layer() -> TraceLayer {
    tower_http::trace::TraceLayer::new_for_http().make_span_with(MakeSpanImpl::new())
}

#[derive(Debug, Clone)]
pub struct MakeSpanImpl;

impl MakeSpanImpl {
    fn new() -> Self {
        Self
    }
}

impl Default for MakeSpanImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> MakeSpan<B> for MakeSpanImpl {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let span = tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
        );

        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor::new(request.headers()))
        });

        use tracing_opentelemetry::OpenTelemetrySpanExt;

        span.set_parent(parent_context);

        span
    }
}

struct HeaderExtractor<'a> {
    headers: &'a http::HeaderMap,
}

impl<'a> HeaderExtractor<'a> {
    fn new(headers: &'a http::HeaderMap) -> Self {
        HeaderExtractor { headers }
    }
}

impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers.keys().map(HeaderName::as_str).collect()
    }
}
