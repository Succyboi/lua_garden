use std::collections::BTreeMap;

use mlua::Table;

use crate::interface::interface_data::InterfaceData;

use super::{parameter::Parameter, Runtime};

#[derive(Clone, PartialEq)]
pub enum RuntimeState {
    Offline,
    Refresh,
    Clear,
    Online
}

#[derive(Clone)]
pub struct RuntimeData {
    pub state : RuntimeState,

    pub sample_rate: f32,
    pub buffer_size: usize,
    pub channels: usize,
    pub run_ms: f32,
    pub input_noise: bool,
    pub clip: bool,

    pub module_name: String,
    pub module_author: String,
    pub module_description: String,

    pub parameters: BTreeMap<String, Parameter>,

    pub change: u32,
    last_interface_change: u32
}

impl RuntimeData {
    pub fn new() -> RuntimeData {
        Self {
            state: RuntimeState::Offline,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,
            run_ms: 0.0,
            input_noise: false,
            clip: true,

            module_name: String::new(),
            module_author: String::new(),
            module_description: String::new(),
            
            parameters: BTreeMap::new(),
            
            change: 0,
            last_interface_change: 0
        }
    }

    pub fn update_from_interface(&mut self, interface_data: &InterfaceData) {
        if self.last_interface_change == interface_data.change { return; }

        self.state = interface_data.runtime_target_state.clone();
        self.clip = interface_data.runtime_clip;
        self.input_noise = interface_data.runtime_input_noise;
    }

    pub fn update_from_runtime(&mut self, runtime: &mut Runtime, interface_data: &InterfaceData) {
        if self.last_interface_change == interface_data.change { return; }

        self.module_name = runtime.name.clone();
        self.module_author = runtime.author.clone();
        self.module_description = runtime.description.clone();
        
        // TODO only copy when refresh from interface is required.
        match &mut runtime.module {
            Some(m) => {
                match m.get_parameters() {
                    Ok(p) => {
                        self.update_parameters(interface_data, p);
                    },
                    Err(e) => {
                        println!("Failed to copy parameters from lua {}", e);
                    }
                }

                match m.update_parameter_value_updates(&mut self.parameters) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Failed to update lua parameter value updates: {}.", e);
                    }
                }
            },
            None => {
                self.parameters.clear();
            }
        }

        self.last_interface_change = interface_data.change;
        self.mark_changed();
    }

    pub fn set_state(&mut self, state: RuntimeState) {
        self.state = state;
        self.mark_changed();
    }

    pub fn mark_changed(&mut self) {
        self.change = self.change + 1;
    }

    fn update_parameters(&mut self, interface_data: &InterfaceData, lua_parameters: Table) {
        self.parameters.clear();

        for lua_parameter in lua_parameters.pairs::<String, Table>() {
            match lua_parameter {
                Ok(lua_parameter_lua) => {
                    match Parameter::new_from_lua(&lua_parameter_lua.1) {
                        Ok(lua_p) => {
                            let parameter = self.parameters.entry(lua_parameter_lua.0).or_insert(lua_p);

                            if interface_data.parameters.contains_key(&parameter.name) {
                                let interface_parameter = interface_data.parameters.get(&parameter.name).expect("No parameter exists at key.");

                                if interface_parameter.changed {
                                    parameter.update_from_parameter(interface_parameter);
                                    parameter.set_changed(interface_parameter.changed);
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to parse parameter \"{name}\": {error}", name = lua_parameter_lua.0, error = e);
                        }
                    };
                },
                Err(e) => {
                    println!("Failed to parse parameters: {}", e);
                }
            }
        }
    }
}