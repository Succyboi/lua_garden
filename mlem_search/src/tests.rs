use mlem_base::runtime::rng::{Rng, RngSplitMix64};
use crate::{feature_extractor, file_utils, metadata_db, vector_db};

#[cfg(test)]

#[test]
fn extract_features() {
    feature_extractor::extract_features(feature_extractor::RunMode::Parallel, "./test_data", |progress| {
        println!("Extracting features... {}%", f32::floor(100.0 * progress)); 
    }).expect("Failed to analyze and build db");
}

#[test]
fn find_nearest() {
    println!("Working directory is {}", file_utils::data_directory().expect("Couldn't get working directory").to_string_lossy());

    let mut features = feature_extractor::extract_features(feature_extractor::RunMode::Parallel, "./test_data", |progress| {
        println!("Extracting features... {}%", f32::floor(100.0 * progress)); 
    }).expect("Failed to analyze and build db");

    let mut metadata_db = metadata_db::MetadataDatabase::new();
    for feature in features.iter_mut() {
        metadata_db.add(feature);
    }
    let last_feature = metadata_db.last().expect("Couldn't get last feature");
    println!("Getting similar features for {path} ({id})", 
        path = last_feature.path(),
        id = last_feature.id_string());

    let vector_db = vector_db::VectorDatabase::new().expect("Failed to create vector database");
    vector_db.add(&features).expect("Failed to add features to vector database");
    for id in vector_db.find_similar(last_feature.id().expect("Feature has no ID"), 8).expect("Failed to get similar features") {
        let feature = metadata_db.get(id).expect("Couldn't get feature from metadata DB");
        println!("Similar sample {path} ({id})", 
            path = feature.path(),
            id = feature.id_string());
    }
}