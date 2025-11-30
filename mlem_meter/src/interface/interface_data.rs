use std::collections::BTreeMap;

use crate::{ runtime::{ parameter::Parameter }, RuntimeData };


#[derive(Clone)]
pub struct InterfaceData {
    pub runtime_clip: bool,
    pub runtime_input_noise: bool,

    pub parameters: BTreeMap<String, Parameter>,

    pub change: u32,
    last_runtime_change: u32
}

impl InterfaceData {
    pub fn new() -> InterfaceData {
        Self {
            runtime_clip: true,
            runtime_input_noise: false,

            parameters: BTreeMap::new(),

            change: 0,
            last_runtime_change: 0
        }
    }

    pub fn update_from_runtime(&mut self, runtime_data: &RuntimeData) {
        if self.last_runtime_change == runtime_data.change { return; }

        self.parameters = runtime_data.parameters.clone();

        self.last_runtime_change = runtime_data.change;
    }

    pub fn mark_changed(&mut self) {
        self.change = self.change + 1;
    }
}