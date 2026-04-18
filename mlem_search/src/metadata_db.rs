use std::{collections::HashMap, ops::Index};
use mlem_base::runtime::rng::{Rng, RngSplitMix64};
use crate::feature::Feature;

pub struct MetadataDatabase {
    features: HashMap<u32, Feature>,
    rng: RngSplitMix64
}

impl MetadataDatabase {
    pub fn new() -> Self {
        return Self {
            features: HashMap::new(),
            rng: RngSplitMix64::new()
        }
    }

    pub fn add(&mut self, feature: &mut Feature) {
        feature.set_id(self.next_id());
        self.features.insert(feature.id().expect("Feature has no id."), feature.clone());
    }

    pub fn get(&self, id: u32) -> Option<&Feature> {
        if !self.features.contains_key(&id) { return None; }

        return Some(&self.features[&id]);
    }

    pub fn last(&mut self) -> Option<&Feature> {
        if self.features.is_empty() { return None;}

        return self.features.values().last();
    }

    fn next_id(&mut self) -> u32  {
        let mut next = self.rng.next_u32();
        
        while self.features.contains_key(&next) {
            next = self.rng.next_u32();
        }

        return next;
    }
}