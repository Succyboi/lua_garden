use log::{error, info};

use crate::{FEATURE_THREADS, FILES_PATH, aggregator::Aggregator, feature_extractor::{self, FeatureExtractor}, metadata_db::MetadataDatabase, vector_db::VectorDatabase};

pub struct AppState {
    pub aggregator: Aggregator,
    pub feature_extractor: FeatureExtractor,
    pub metadata_db: MetadataDatabase,
    pub vector_db: VectorDatabase
}

impl AppState {
    pub fn new() -> Result<Self, String> {
        let mut new = Self {
            aggregator: Aggregator::new(),
            feature_extractor: FeatureExtractor::new(FEATURE_THREADS),
            metadata_db: MetadataDatabase::new(),
            vector_db: VectorDatabase::new().expect("Couldn't create vector database")
        };

        let default_paths = feature_extractor::get_audio_files_form_dir(FILES_PATH);
        for path in default_paths {
            let id = new.metadata_db.next_id();
            new.feature_extractor.extract_feature(&path, Some(id));
        }

        new.aggregator.connect()?;

        return Ok(new); 
    }

    pub fn update(&mut self) {
        if let Some(features) = self.feature_extractor.receive_features() {
            for mut feature in features {
                self.metadata_db.add(&mut feature);
                match self.vector_db.add(&feature){
                    Ok(_) => (),
                    Err(e) => {
                        error!("Couldn't add feature to vector database: {}", e);
                    }
                }
            }
        }
    }
}