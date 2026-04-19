#[cfg(test)]
mod tests {
    use crate::metadata_db::MetadataDatabase;
    use crate::{FEATURE_THREADS, feature_extractor::{self, FeatureExtractor}, metadata_db, vector_db};

    #[test]
    fn extract_features() {
        let root_paths = feature_extractor::get_audio_files_form_dir("./test_data");
        let feature_extractor = FeatureExtractor::new(FEATURE_THREADS);
        let mut metadata_database = MetadataDatabase::new();

        for path in root_paths {
            feature_extractor.extract_feature(&path);
        }

        while feature_extractor.working() {
            if let Some(features) = feature_extractor.receive_features() {
                for mut feature in features {
                    metadata_database.add(&mut feature);

                    println!("Extracted feature {path} ({id})", 
                        path = feature.path(),
                        id = feature.id_string());
                }
            }
        }
    }

    #[test]
    fn find_nearest() {
        let root_paths = feature_extractor::get_audio_files_form_dir("./test_data");
        let feature_extractor = FeatureExtractor::new(FEATURE_THREADS);
        let mut metadata_db = MetadataDatabase::new();
        let vector_db = vector_db::VectorDatabase::new().expect("Failed to create vector database");

        for path in root_paths {
            feature_extractor.extract_feature(&path);
        }

        while feature_extractor.working() {
            if let Some(features) = feature_extractor.receive_features() {
                for mut feature in features {
                    metadata_db.add(&mut feature);
                    vector_db.add(&feature).expect("Failed to add features to vector database");

                    println!("Extracted feature {path} ({id})", 
                        path = feature.path(),
                        id = feature.id_string());
                }
            }
        }

        let last_feature = metadata_db.random().expect("Couldn't get last feature");
        println!("Getting similar features for {path} ({id})", 
            path = last_feature.path(),
            id = last_feature.id_string());

        for similar in vector_db.find_similar(last_feature.id().expect("Feature has no ID"), 1000).expect("Failed to get similar features") {
            let feature = metadata_db.get(similar.0).expect("Couldn't get feature from metadata DB");
            println!("Similar feature {path} ({id}) at {similarity}% similar", 
                path = feature.path(),
                id = feature.id_string(),
                similarity = 100.0 * (1.0 - similar.1));
        }
    }
}
