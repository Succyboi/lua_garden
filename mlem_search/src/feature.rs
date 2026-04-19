use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Feature {
    feature_vector: Vec<f32>,
    source_file: String,
    id: Option<u32>,
}

impl Feature {
    pub fn new(feature_vector: Vec<f32>, source_file: String, id: Option<u32>) -> Self {
        Self {
            feature_vector,
            source_file,
            id,
        }
    }

    pub fn vector(&self) -> &[f32] {
        &self.feature_vector
    }

    pub fn path(&self) -> &str {
        &self.source_file
    }

    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn id_string(&self) -> String {
        let id = match self.id() {
            Some(i) => i,
            None => &u32::default()
        };

        return format!("{:02X?}", id);
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }
}