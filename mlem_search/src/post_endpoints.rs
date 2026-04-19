use std::sync::Mutex;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, get, post, web::{self, Data}};
use serde::Deserialize;
use crate::{FILES_PATH, appstate::AppState};

#[derive(MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    files: Vec<TempFile>,
}

#[post("/upload")]
pub async fn upload(app_state: Data<Mutex<AppState>>, MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let mut app_state = app_state.lock().unwrap();
    app_state.update();

    for file in form.files {
        let path = format!("{files}/{file}", files = FILES_PATH, file = file.file_name.unwrap());
        if let Ok(_) = file.file.persist(&path) {
            app_state.feature_extractor.extract_feature(&path);
        }
    }
 
    return HttpResponse::Ok();
}