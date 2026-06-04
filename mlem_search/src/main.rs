#![feature(iter_array_chunks)]

mod feature;
mod feature_extractor;
mod metadata_db;
mod vector_db;
mod tests;
mod appstate;
mod aggregator;
mod get_endpoints;
mod post_endpoints;
mod util;

use actix_cors::Cors;
use env_logger::Env;
use log::{ info, warn, error };
use std::{io::Error, sync::Mutex, time::SystemTime};
use actix_files::Files;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{App, HttpResponse, HttpServer, Responder, middleware, web::{self, Data, get, post, resource}};
use crate::{appstate::AppState, get_endpoints::{aggregate_search, all, feature_id, random, similar, state, web}, post_endpoints::upload};

const API_PORT: usize = 16769;
const API_WORKERS: usize = 16;

const FEATURE_THREADS: usize = 16;
const FEATURE_DIMENSIONS: usize = 60;
const FEATURE_SAMPLE_RATE: usize = 22050;

const FILES_PATH: &str = "static";
const WEB_PATH: &str = "web";
const UPLOADS_PATH: &str = "static/uploads";
const MAX_DB_SIZE: usize = 1024 * 1024 * 1024 * 1024;

const AGGREGATOR_USERNAME: &str = "mlem_search";
const AGGREGATOR_PASSWORD: &str = "D4KmIvJ5OtqPLY";
const AGGREGATOR_TIMEOUT: usize = 10;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Data::new(Mutex::new(AppState::new().map_err(|e| Error::new(std::io::ErrorKind::NotConnected, e))?));

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server at http://127.0.0.1:{}", API_PORT);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .app_data(TempFileConfig::default().directory(UPLOADS_PATH))
            .service(Files::new("/static", FILES_PATH))
            
            .service(state)
            .service(all)
            .service(random)
            .service(similar)
            .service(feature_id)
            .service(aggregate_search)
            
            .service(Files::new("/web", WEB_PATH))
            .service(resource("/upload")
                .route(get().to(web))
                .route(post().to(upload)))
    })
    .bind(format!("127.0.0.1:{}", API_PORT))?
    .workers(API_WORKERS)
    .run()
    .await
}

// TODO
// - Figure out why certain uploads fail
// - Proper web interface
// - Limiting: File sizes, audio duration, upload rates, file exist checks
// - Max database enforcement
// - Reporting system
// - Database implementation
// - Optimization to prevent updating app state on request threads
// - String search for sample names?
// - Download credits based on uploads?