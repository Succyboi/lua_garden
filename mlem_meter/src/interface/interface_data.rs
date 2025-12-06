use std::collections::BTreeMap;

use crate::{ RuntimeData };


#[derive(Clone)]
pub struct InterfaceData {
    pub meter_id: usize,
}

impl InterfaceData {
    pub fn new() -> InterfaceData {
        Self {
            meter_id: 0,
        }
    }

    pub fn update_from_runtime(&mut self, runtime_data: &RuntimeData) {
        
    }

    pub fn reset_meter(&mut self) {
        self.meter_id = self.meter_id + 1;
    }
}