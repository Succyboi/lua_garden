use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Clone)]
pub struct Feature {
    vector: Vec<f32>,
    path: String,
    id: Option<u32>,
}

impl Feature {
    pub fn new(vector: Vec<f32>, path: String, id: Option<u32>) -> Self {
        Self {
            vector,
            path,
            id,
        }
    }

    pub fn vector(&self) -> &[f32] {
        &self.vector
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn id_string(&self) -> String {
        let id = match self.id() {
            Some(i) => i,
            None => &u32::default()
        };

        return format!("{}", id);
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }
}

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Feature", 2)?;
        state.serialize_field("id", &self.id.unwrap())?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}