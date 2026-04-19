use std::num::NonZeroUsize;

use heed::Env;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::feature::Feature;
use crate::{FEATURE_DIMENSIONS, MAX_DB_SIZE};
use arroy::distances::Angular;
use arroy::{Database as ArroyDatabase, Reader, Writer};

unsafe fn create_env() -> Result<Env, String> {
    let dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let env = unsafe {
        heed::EnvOpenOptions::new()
            .map_size(MAX_DB_SIZE)
            .open(&dir)
    }
    .map_err(|_| format!("Failed to open database from {}", dir.to_string_lossy()))?;
    Ok(env)
}

pub struct VectorDatabase {
    db: ArroyDatabase<Angular>,
}

impl VectorDatabase {
    pub fn new() -> Result<VectorDatabase, String> {
        let env = unsafe { create_env()? };
        // TODO: this can probably be a read txn
        let mut write_txn = env.write_txn().map_err(|e| e.to_string())?;
        let db: ArroyDatabase<Angular> = env
            .create_database(&mut write_txn, None)
            .map_err(|e| e.to_string())?;
        db.clear(&mut write_txn).map_err(|e| e.to_string())?;
        let writer = Writer::<Angular>::new(db, 0, FEATURE_DIMENSIONS);
        let mut rng = StdRng::from_entropy();
        // Note: we still need to call build() after loading the db from disk. Even if
        // the index was previous built.
        writer
            .build(&mut write_txn, &mut rng, None)
            .map_err(|e| e.to_string())?;
        write_txn.commit().map_err(|e| e.to_string())?;
        Ok(VectorDatabase { db })
    }

    pub fn add(&self, feature: &Feature) -> Result<(), String> {
        let env = unsafe { create_env()? };
        let mut write_txn = env.write_txn().map_err(|e| e.to_string())?;

        let index = 0;
        let writer = Writer::<Angular>::new(self.db, index, FEATURE_DIMENSIONS);
        let id = feature.id().unwrap();
        // Write to the arroy vector db using the id from the sqlite table
        writer
            .add_item(&mut write_txn, id as u32, feature.vector())
            .map_err(|e| e.to_string())?;

        // Build index
        let mut rng = StdRng::from_entropy();
        let num_trees = None;
        writer
            .build(&mut write_txn, &mut rng, num_trees)
            .map_err(|e| e.to_string())?;

        // Commit the built index to the db
        write_txn.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Returns a vector of file ids to the top k similar results
    pub fn find_similar(&self, id: u32, num_results: usize) -> Result<Vec<(u32, f32)>, String> {
        let env = unsafe { create_env()? };
        let rtxn = env.read_txn().map_err(|e| e.to_string())?;
        let index = 0;
        let reader = Reader::<Angular>::open(&rtxn, index, self.db).map_err(|e| e.to_string())?;

        // You can increase the quality of the results by forcing arroy to search into more nodes.
        // This multiplier is arbitrary but basically the higher, the better the results, the slower the query.
        let search_k = NonZeroUsize::new(num_results * reader.n_trees() * 15);

        // Similar searching can be achieved by requesting the nearest neighbors of a given item.
        let search_results = reader
            .nns_by_item(&rtxn, id, num_results + 1, search_k, None)
            .map_err(|e| e.to_string())?
            .ok_or("Unexpected similarity search error".to_string())?
            .iter()
            .map(|result| (result.0, result.1))
            .filter(|i| id != i.0)
            .collect();
        Ok(search_results)
    }
}
