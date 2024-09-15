use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{SpanKind, Status};
use opentelemetry::{global, trace::Tracer};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout::SpanExporter;

fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);
}

fn call_tracing(req_body: Option<String>, e: Option<Box<dyn std::error::Error>>) {
    let tracer = global::tracer("main");

    let mut span = if let Some(req_body) = req_body {
        tracer
            .span_builder(format!("{}", req_body))
            .with_kind(SpanKind::Server)
            .start(&tracer)
    } else {
        tracer
            .span_builder("None")
            .with_kind(SpanKind::Server)
            .start(&tracer)
    };

    if let Some(e) = e {
        span.set_status(Status::error(e.to_string()));
    } else {
        span.set_status(Status::Ok);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracer();

    HttpServer::new(|| App::new().service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    call_tracing(Some(req_body.clone()), None);

    HttpResponse::Ok().body(req_body)
}
