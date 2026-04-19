use crate::{FEATURE_THREADS, feature_extractor::{self, FeatureExtractor}, metadata_db::MetadataDatabase, vector_db::VectorDatabase};

pub struct AppState {
    pub feature_extractor: FeatureExtractor,
    pub metadata_db: MetadataDatabase,
    pub vector_db: VectorDatabase
}

impl AppState {
    pub fn new() -> Self {
        let new = Self {
            feature_extractor: FeatureExtractor::new(FEATURE_THREADS),
            metadata_db: MetadataDatabase::new(),
            vector_db: VectorDatabase::new().expect("Couldn't create vector database")
        };

        let default_paths = feature_extractor::get_audio_files_form_dir("static/default");
        for path in default_paths {
            new.feature_extractor.extract_feature(&path);
        }

        return new; 
    }

    pub fn update(&mut self) {
        if let Some(features) = self.feature_extractor.receive_features() {
            for mut feature in features {
                self.metadata_db.add(&mut feature);
                let _ = self.vector_db.add(&feature);

                println!("Extracted feature {path} ({id})", 
                    path = feature.path(),
                    id = feature.id_string());
            }
        }
    }
}