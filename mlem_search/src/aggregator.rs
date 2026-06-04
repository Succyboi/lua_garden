use log::info;
use soulseek_rs::{Client, ClientSettings, PeerAddress};
use std::{io::Error, path, time::Duration};

use crate::{AGGREGATOR_PASSWORD, AGGREGATOR_TIMEOUT, AGGREGATOR_USERNAME, UPLOADS_PATH, feature_extractor::{FeatureExtractor, has_supported_extension}};

pub struct Aggregator {
    client: Client
}

impl Aggregator {
    pub fn new() -> Self {
        let new = Self {
            client: Client::new(AGGREGATOR_USERNAME, AGGREGATOR_PASSWORD),
        };

        return new; 
    }

    pub fn connect(&mut self) -> Result<(), String> {
        self.client.connect();
        self.client.login().map_err(|e| e.to_string())?;

        return Ok(());
    }

    pub fn aggregate(&self, term: impl Into<String>, amount: usize) -> Result<Vec<String>, String> {
        let term = term.into();
        let mut downloads = Vec::new();
        let results = self.client.search(&term, Duration::from_secs(AGGREGATOR_TIMEOUT as u64)).map_err(|e| e.to_string())?;
        let path = UPLOADS_PATH;

        for result in results.iter() {
            if downloads.len() >= amount { break; }
            if result.files.is_empty() { continue; }

            for file in result.files.iter() {
                if downloads.len() >= amount { break; }
                if !has_supported_extension(file.name.clone()) { continue; }

                self.client.download(
                    file.name.clone(),
                    file.username.clone(),
                    file.size,
                    String::from(UPLOADS_PATH),
                ).map_err(|e| e.to_string())?;
                
                info!("Aggregated \"{file}\" for term \"{term}\"", file = file.name);
                downloads.push(format!("{path}/{file}", file = file.name));
            }
        }

        return Ok(downloads);
    }
}