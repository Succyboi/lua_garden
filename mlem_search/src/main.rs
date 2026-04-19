#![feature(iter_array_chunks)]

mod feature;
mod feature_extractor;
mod metadata_db;
mod vector_db;
mod tests;
mod appstate;
mod get_endpoints;
mod post_endpoints;

use std::{sync::Mutex, time::SystemTime};
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::{self, Data, post}};
use crate::{appstate::AppState, get_endpoints::{random, similar, state}, post_endpoints::upload};

const API_PORT: usize = 16769;
const FEATURE_THREADS: usize = 4;
const FEATURE_DIMENSIONS: usize = 60;
const FEATURE_SAMPLE_RATE: usize = 22050;
const FILES_PATH: &str = "static";
const WEB_PATH: &str = "web";
const MAX_DB_SIZE: usize = 2 * 1024 * 1024 * 1024;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Data::new(Mutex::new(AppState::new()));

    println!("Starting server at http://127.0.0.1:{}", API_PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(Files::new("/static", FILES_PATH))
            .service(Files::new("/web", WEB_PATH).show_files_listing())

            .service(state)
            .service(random)
            .service(similar)
            
            .service(upload)
    })
    .bind(format!("127.0.0.1:{}", API_PORT))?
    .run()
    .await
}