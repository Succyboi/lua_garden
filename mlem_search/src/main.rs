#![feature(iter_array_chunks)]

mod feature;
pub mod feature_extractor;
pub mod metadata_db;
pub mod vector_db;
pub mod tests;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

const API_PORT: usize = 6769;
const FEATURE_THREADS: usize = 4;
const FEATURE_DIMENSIONS: usize = 60;
const FEATURE_SAMPLE_RATE: usize = 22050;
const MAX_DB_SIZE: usize = 2 * 1024 * 1024 * 1024;

// Health check endpoint - useful for load balancers and monitoring
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    println!("Starting server at http://127.0.0.1:{}", API_PORT);
    
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}