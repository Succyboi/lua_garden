use std::{collections::{HashMap, VecDeque}, ops::Index};
use indexmap::IndexMap;
use mlem_base::runtime::rng::{self, Rng, RngSplitMix64};
use crate::feature::Feature;

pub struct MetadataDatabase {
    features: IndexMap<u32, Feature>,
    rng: RngSplitMix64
}

impl MetadataDatabase {
    pub fn new() -> Self {
        return Self {
            features: IndexMap::new(),
            rng: RngSplitMix64::new()
        }
    }

    pub fn add(&mut self, feature: &mut Feature) {
        let id = self.next_id();
        feature.set_id(id);

        self.features.insert(id, feature.clone());
    }

    pub fn get(&self, id: u32) -> Option<&Feature> {
        if !self.features.contains_key(&id) { return None; }

        return Some(&self.features[&id]);
    }

    pub fn first(&self) -> Option<&Feature> {
        if let Some(feature) = self.features.first() {
            return Some(feature.1);
        }
        
        return None; 
    }

    pub fn last(&self) -> Option<&Feature> {
        if let Some(feature) = self.features.last() {
            return Some(feature.1);
        }
        
        return None; 
    }

    pub fn random(&mut self) -> Option<&Feature> {
        if self.features.is_empty() { return None;}

        let index = self.rng.range_usize(0..self.features.len());
        if let Some(feature) = self.features.get_index(index) {
            return Some(feature.1);
        }

        return None;
    }

    fn next_id(&mut self) -> u32  {
        let mut next = self.rng.next_u32();
        
        while self.features.contains_key(&next) {
            next = self.rng.next_u32();
        }

        return next;
    }
}