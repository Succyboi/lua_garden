use std::collections::BTreeMap;

use super::InterfaceMode;
use crate::{ runtime::{library, module_content::ModuleContent, parameter::Parameter, runtime_data::RuntimeState, workspace::Workspace}, RuntimeData };


#[derive(Clone)]
pub struct InterfaceData {
    pub mode: InterfaceMode,
    pub workspace: Option<Workspace>,
    pub draft_content: ModuleContent,

    pub runtime_target_state: RuntimeState,
    pub runtime_clip: bool,
    pub runtime_input_noise: bool,

    pub parameters: BTreeMap<String, Parameter>,

    pub change: u32,
    last_runtime_change: u32
}

impl InterfaceData {
    pub fn new() -> InterfaceData {
        Self {
            mode: InterfaceMode::Draft,
            workspace: None,
            draft_content: library::MODULE_EXAMPLES[0].0.to_module_content(),

            runtime_target_state: RuntimeState::Offline,
            runtime_clip: true,
            runtime_input_noise: false,

            parameters: BTreeMap::new(),

            change: 0,
            last_runtime_change: 0
        }
    }

    pub fn update_from_runtime(&mut self, runtime_data: &RuntimeData) {
        if self.last_runtime_change == runtime_data.change { return; }

        self.runtime_target_state = runtime_data.state.clone();
        self.parameters = runtime_data.parameters.clone();

        self.last_runtime_change = runtime_data.change;
    }

    pub fn set_runtime_target_state(&mut self, runtime_target_state: RuntimeState) {
        self.runtime_target_state = runtime_target_state;
        self.mark_changed();
    }

    pub fn set_runtime_clip(&mut self, runtime_clip: bool) {
        self.runtime_clip = runtime_clip;
        self.mark_changed();
    }

    pub fn set_runtime_input_noise(&mut self, runtime_input_noise: bool) {
        self.runtime_input_noise = runtime_input_noise;
        self.mark_changed();
    }

    pub fn mark_changed(&mut self) {
        self.change = self.change + 1;
    }
}