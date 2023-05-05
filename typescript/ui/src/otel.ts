import {
  BatchSpanProcessor,
} from '@opentelemetry/sdk-trace-base';
import { WebTracerProvider, TraceIdRatioBasedSampler } from "@opentelemetry/sdk-trace-web";
// import { DocumentLoadInstrumentation } from "@opentelemetry/instrumentation-document-load";
import { ZoneContextManager } from "@opentelemetry/context-zone";
import { registerInstrumentations } from '@opentelemetry/instrumentation';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { SemanticResourceAttributes } from '@opentelemetry/semantic-conventions' 
import { Resource } from '@opentelemetry/resources'
import { propagation } from '@opentelemetry/api';
// import { OTTracePropagator } from '@opentelemetry/propagator-ot-trace';
import { getWebAutoInstrumentations } from '@opentelemetry/auto-instrumentations-web'
import { W3CTraceContextPropagator } from '@opentelemetry/core';


const init = () =>  {

  propagation.setGlobalPropagator(new W3CTraceContextPropagator())

  const samplingRatio = 1.0

  const resource = new Resource({ 
    [SemanticResourceAttributes.SERVICE_VERSION]: "0.1.0",
    [SemanticResourceAttributes.SERVICE_NAMESPACE]: 'opentelemetry-workspace',
    [SemanticResourceAttributes.SERVICE_NAME]: 'ui',

  });  

  const collectorOptions = {
    url: 'http://localhost:4318/v1/traces',
    headers: {},
    concurrencyLimit: 10,
  };

 
  const provider = new WebTracerProvider({
    resource,
    sampler: new TraceIdRatioBasedSampler(samplingRatio),

  });

  const exporter = new OTLPTraceExporter(collectorOptions);

  provider.addSpanProcessor(new BatchSpanProcessor(exporter, {
    maxQueueSize: 100,
    maxExportBatchSize: 10,
    scheduledDelayMillis: 500,
    exportTimeoutMillis: 30000,
  }));

  provider.register({
    contextManager: new ZoneContextManager(),
    propagator: new W3CTraceContextPropagator(),
  });

  registerInstrumentations({
    instrumentations: [
      getWebAutoInstrumentations({
        '@opentelemetry/instrumentation-document-load': {
          enabled: true, 
        },      
        '@opentelemetry/instrumentation-fetch': { 
          enabled: true,
          // urls which should include trace headers when origin doesn't match
          propagateTraceHeaderCorsUrls: [
            'http://localhost:8000/graphql',
          ]
           },
        '@opentelemetry/instrumentation-user-interaction': {
          enabled: true
        },
        '@opentelemetry/instrumentation-xml-http-request': {
          enabled: true
        },
      })
    ],
  });
  
  console.log("Init otel...")

};

export default init