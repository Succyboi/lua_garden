use std::collections::BTreeMap;

use crate::{ runtime::{ parameter::Parameter }, RuntimeData };


#[derive(Clone)]
pub struct InterfaceData {
    pub runtime_clip: bool,
    pub runtime_input_noise: bool,

    pub parameters: BTreeMap<String, Parameter>,
}

impl InterfaceData {
    pub fn new() -> InterfaceData {
        Self {
            runtime_clip: true,
            runtime_input_noise: false,

            parameters: BTreeMap::new(),
        }
    }

    pub fn update_from_runtime(&mut self, runtime_data: &RuntimeData) {
    }
}