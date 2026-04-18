#![feature(iter_array_chunks)]

use std::time::Instant;
use feature::Feature;
use vector_db::VectorDatabase;

mod feature;
pub mod feature_extractor;
mod file_utils;
pub mod metadata_db;
pub mod vector_db;
pub mod tests;

const FEATURE_THREADS: usize = 4;
const FEATURE_DIMENSIONS: usize = 60;
const FEATURE_SAMPLE_RATE: usize = 22050;
const MAX_DB_SIZE: usize = 2 * 1024 * 1024 * 1024;

pub fn analyze_and_build_db(asset_dir: &str, progress_callback: impl Fn(f32)) -> Result<VectorDatabase, String> {
    let start_time = Instant::now();

    let features: Vec<Feature> = feature_extractor::extract_features(
        feature_extractor::RunMode::Parallel,
        asset_dir,
        progress_callback,
    )?;

    let elapsed = start_time.elapsed();
    println!("Took {:.1?} to extract features", elapsed);

    let start_time = Instant::now();

    // Combine previously cached features with the new ones
    let db = VectorDatabase::new()?;
    db.add(&features)?;
    let elapsed = start_time.elapsed();
    println!("Took {:.1?} to build database", elapsed);

    Ok(db)
}

/*
pub fn find_similar(source_id: u32, num_results: usize) -> Result<Vec<AudioFile>, String> {
    // Otherwise, load the existing db from disk and query it
    let vec_db = VectorDatabase::load_from_disk()?;
    let ids = vec_db.find_similar(source_id, num_results)?;
    let md_db = MetadataDatabase::load_from_disk()?;
    md_db.get_audio_files_for_ids(&ids)
}

pub fn list_audio_files(start_offset: u32, num_results: u32) -> Result<Vec<AudioFile>, String> {
    let db = MetadataDatabase::load_from_disk()?;
    db.list_audio_files(start_offset, Some(num_results))
}
*/