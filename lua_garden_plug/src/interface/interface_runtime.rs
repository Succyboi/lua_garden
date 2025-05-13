use super::{interface_module::InterfaceModule, DEFAULT_SPACE};

use nih_plug_egui::egui::{self, Ui} ;
use crate::{ runtime::{ parameter::Parameter, runtime_data::RuntimeData }, InterfaceData };

const PARAMETER_GRID_ID: &str = "Central/Parameters";

pub struct InterfaceRuntime {
    pub module: Option<InterfaceModule>,
    pub view: InterfaceRuntimeView
}

#[derive(PartialEq)]
pub enum InterfaceRuntimeView {
    Interface,
    Parameters
}

impl InterfaceRuntime {
    pub fn new() -> InterfaceRuntime {
        Self {
            module: None,
            view: InterfaceRuntimeView::Interface
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, runtime_data: &RuntimeData, interface_data: &mut InterfaceData) {
        match self.view {
            InterfaceRuntimeView::Interface => {
                // TODO
                ui.label("To be implemented.");
            },
            InterfaceRuntimeView::Parameters => {
                self.draw_parameters(ui, runtime_data, interface_data);
            }
        }
    }

    pub fn draw_parameters(&mut self, ui: &mut Ui, runtime_data: &RuntimeData, interface_data: &mut InterfaceData) {
        ui.label(format!("Module \"{name}\" has {parameter_count} parameter(s):", 
            name = runtime_data.module_name, 
            parameter_count = interface_data.parameters.len()));

        let mut changed = false;
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                egui::Grid::new(PARAMETER_GRID_ID)
                    .num_columns(2)
                    .spacing([DEFAULT_SPACE * 4.0, DEFAULT_SPACE])
                    .show(ui, |ui| {
                    for parameter in &mut interface_data.parameters {
                        self.draw_parameter(ui, parameter.1);

                        if parameter.1.changed {
                            changed = true;
                        }
                    }
                });
        });

        if changed {
            interface_data.mark_changed();
        }
    }

    fn draw_parameter(&mut self, ui: &mut Ui, parameter: &mut Parameter) {
        ui.label(&parameter.name);
        parameter.draw(ui);
        ui.end_row();
    }
}