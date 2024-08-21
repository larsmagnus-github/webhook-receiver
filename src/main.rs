use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;
use log::info;
use env_logger;

#[derive(Debug, Deserialize)]
struct WebhookPayload {
    event_type: String,
    timestamp: String,
    data: serde_json::Value,  // Flexible for different types of data
}

#[post("/webhook")]
async fn webhook_handler(payload: web::Json<WebhookPayload>) -> impl Responder {
    info!("Received webhook: {:?}", payload);

    HttpResponse::Ok().body("Webhook received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(webhook_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}