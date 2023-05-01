import os
from fastapi import FastAPI
from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
from opentelemetry.sdk.extension.aws.trace import AwsXRayIdGenerator
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.trace.span import Span
from opentelemetry.semconv.resource import ResourceAttributes
from opentelemetry.sdk.trace.sampling import Sampler,ParentBased, TraceIdRatioBased

def _configure_sampler() -> Sampler:
    ratio = float(os.getenv("OTEL_TRACE_SAMPLING_RATIO", "1.0"))

    return ParentBased(TraceIdRatioBased(ratio))

def _configure_tracer(
    otlp_exporter_endpoint: str,
    service: str,
    version: str,
    deployment_environment: str,
):
    """Construct trace pipeline then return tracer.
    Telemetry data is exported to the endpoint specified in the argument
    """
    exporter = OTLPSpanExporter(endpoint=otlp_exporter_endpoint, insecure=True)
    span_processor = BatchSpanProcessor(exporter)

    resource = Resource.create(
        {
            ResourceAttributes.SERVICE_NAME: service,
            ResourceAttributes.SERVICE_VERSION: version,
            ResourceAttributes.DEPLOYMENT_ENVIRONMENT: deployment_environment,
        }
    )

    sampler = _configure_sampler()
    
    provider = TracerProvider(resource=resource, id_generator=AwsXRayIdGenerator(), sampler=sampler)
    provider.add_span_processor(span_processor)

    trace.set_tracer_provider(provider)

    return trace.get_tracer(__name__)

def _server_request_hook(span: Span, scope: dict):
    return


def _client_request_hook(span: Span, scope: dict):
    return


def _client_response_hook(span: Span, message: dict):
    return

tracer = trace.get_tracer(__name__)

def enable_opentelemetry(
    app: FastAPI,
    service: str,
    version: str,
    otlp_exporter_endpoint: str,
    deployment_environment: str = "unknown",
) -> None:
    global tracer
    tracer = _configure_tracer(
        otlp_exporter_endpoint=otlp_exporter_endpoint,
        service=service,
        version=version,
        deployment_environment=deployment_environment,
    )

    FastAPIInstrumentor.instrument_app(
        app,
        excluded_urls="",  # ex. "internal/*,healthcheck"
        server_request_hook=_server_request_hook,
        client_request_hook=_client_request_hook,
        client_response_hook=_client_response_hook,
        tracer_provider=trace.get_tracer_provider(),
    )