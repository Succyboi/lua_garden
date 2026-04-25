use std::sync::Mutex;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, get, post, web::{self, Data}};
use log::info;
use crate::{FILES_PATH, UPLOADS_PATH, appstate::AppState};

#[derive(MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(serde::Serialize)]
pub struct UploadedFile {
    path: String,
    id: u32
}

pub async fn upload(app_state: Data<Mutex<AppState>>, MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let mut app_state = app_state.lock().unwrap();
    app_state.update();
    let mut paths = Vec::new();

    for file in form.files {
        let path = format!("{files}/({id:02X?}) {file}", files = UPLOADS_PATH, id = file.size, file = file.file_name.unwrap());
        
        if let Ok(_) = file.file.persist(&path) {
            let id = app_state.metadata_db.next_id();
            app_state.feature_extractor.extract_feature(&path, Some(id));

            paths.push(UploadedFile {
                path, 
                id
            });
        }
    }

    return HttpResponse::Created().json(serde_json::json!({
            "files": paths,
        }));
}