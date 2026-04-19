use std::sync::Mutex;
use actix_web::{HttpResponse, Responder, get, web::{Data, Path}};
use serde::Serialize;
use crate::{appstate::AppState, feature::Feature};

// General state
#[get("/state")]
async fn state(app_state: Data<Mutex<AppState>>) -> impl Responder {
    let mut app_state = app_state.lock().unwrap();
    app_state.update();

    let features = app_state.metadata_db.len();

    return HttpResponse::Ok().json(serde_json::json!({
        "features": features,
    }));
}

// Random feature
#[get("/random")]
async fn random(app_state: Data<Mutex<AppState>>) -> impl Responder {
    let mut app_state = app_state.lock().unwrap();
    app_state.update();

    let feature = app_state.metadata_db.random();

    match feature {
        Some(f) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "feature": f,
            }));
        },
        None => {
            return HttpResponse::NotAcceptable().finish();
        }
    }
}

#[derive(Serialize)]
struct SimilarResponse {
    feature: Feature,
    distance: f32
}

// Get similar features
#[get("/similar/{id}-{count}")]
async fn similar(app_state: Data<Mutex<AppState>>, path: Path<(u32, usize)>) -> impl Responder {
    let mut app_state = app_state.lock().unwrap();
    app_state.update();

    let id = path.into_inner();

    if let Some(_) = app_state.metadata_db.get(id.0) {
        let mut similar_features = Vec::new();

        for similar in app_state.vector_db.find_similar(id.0, id.1).expect("Failed to get similar features") {
            let feature = app_state.metadata_db.get(similar.0);

            if let Some(feature) = feature {
                similar_features.push(SimilarResponse {
                    feature: feature.clone(),
                    distance: similar.1
                });
            }
        }

        return HttpResponse::Ok().json(serde_json::json!({
            "feature": similar_features,
        }));
    } else {
        return HttpResponse::NotFound().finish();
    }
}