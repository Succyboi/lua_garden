pub mod interface_data;
pub mod interface_utils;
pub mod parameter;

use std::{ hash::Hash, sync::{ Arc, RwLock }, time::Duration };
use mlem_egui_themes::Theme;
use nih_plug::{formatters::v2s_f32_gain_to_db, prelude::*, util::gain_to_db};
use nih_plug_egui::{ egui::{ self, Context, Ui }, EguiState };
use interface_data::InterfaceData;
use crate::{ ConsoleReceiver, PluginImplementationParams, RuntimeData, consts, interface::interface_utils::{help_label, parameter_label} };

const DEFAULT_SPACE: f32 = 4.0;
const LABEL_WIDTH: f32 = 64.0;
const TOP_ID: &str = "Top";
const DEFAULT_MENU_WIDTH: f32 = 64.0;
const ABOUT_MENU_WIDTH: f32 = consts::WINDOW_SIZE_WIDTH as f32 - DEFAULT_MENU_WIDTH - 32.0;
const ABOUT_LICENSE_SCROLL_HEIGHT: f32 = 128.0;
const CONSOLE_MAIN_ID: &str = "Central/Console/Main";
const CONSOLE_ICON: &str = "\u{E47E}";

pub struct Interface {
    pub console: ConsoleReceiver,

    show_console: bool,

    theme: usize,
    themes: [mlem_egui_themes::Theme; 4],
}

impl Interface {
    pub fn new() -> Interface {
        return Self {
            console: ConsoleReceiver::new(),

            show_console: false,

            theme: 0,
            themes: [
                mlem_egui_themes::garden_night(),
                mlem_egui_themes::garden_day(),
                mlem_egui_themes::garden_gameboy(),
                mlem_egui_themes::garden_playdate()
            ]
        };
    }

    pub fn create_interface(self, editor_state: Arc<EguiState>, params: Arc<PluginImplementationParams>, runtime_data_lock: Arc<RwLock<RuntimeData>>, interface_data_lock: Arc<RwLock<InterfaceData>>) -> Option<Box<dyn Editor>> {
        let interface_lock = Arc::from(RwLock::from(self));
        let interface_lock_build = interface_lock.clone();
        let interface_lock_update = interface_lock.clone();
        let runtime_data_lock_build = runtime_data_lock.clone();
        let runtime_data_lock_update = runtime_data_lock.clone();
        let interface_data_lock_build = interface_data_lock.clone();
        let interface_data_lock_update = interface_data_lock.clone();
        let params_build = params.clone();
        let params_update = params.clone();

        return nih_plug_egui::create_egui_editor(
            editor_state,
            (),
            move |egui_ctx, _state| {
                let params_build = params_build.clone();
                let interface = interface_lock_build.clone();
                let runtime_data = runtime_data_lock_build.clone();
                let interface_data = interface_data_lock_build.clone();

                interface.write().unwrap().build_interface(egui_ctx, _state, params_build, runtime_data, interface_data);
            },
            move |egui_ctx, _setter, _state| {
                let params_update = params_update.clone();
                let interface = interface_lock_update.clone();
                let runtime_data = runtime_data_lock_update.clone();
                let interface_data = interface_data_lock_update.clone();

                interface.write().unwrap().draw_interface(egui_ctx, _setter, _state, params_update, runtime_data, interface_data);
            },
        );
    }

    fn build_interface(&mut self, egui_ctx: &Context, _state: &mut (), _params: Arc<PluginImplementationParams>, _runtime_data: Arc<RwLock<RuntimeData>>, _interface_data: Arc<RwLock<InterfaceData>>) {
        mlem_egui_themes::set_theme(egui_ctx, self.get_theme());

        self.console.log(format!("{name} \"{description}\" v{version} {build_type} ({id}).", name = consts::NAME, description = consts::DESCRIPTION, version = consts::VERSION, build_type = consts::BUILD_TYPE, id = consts::BUILD_ID));
        self.console.log(format!("By {}", consts::AUTHORS));
        self.console.log(format!("{}", consts::MOTD));
    }
    
    fn draw_interface(&mut self, egui_ctx: &Context, _setter: &ParamSetter, _state: &mut (), _params: Arc<PluginImplementationParams>, runtime_data: Arc<RwLock<RuntimeData>>, interface_data: Arc<RwLock<InterfaceData>>) {    
        let runtime_data = runtime_data.read().unwrap().clone();
        let mut interface_data = interface_data.write().unwrap();
        
        interface_data.update_from_runtime(&runtime_data);

        egui::TopBottomPanel::top(TOP_ID).show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                self.draw_about_button(ui);
                self.draw_darkmode_toggle(egui_ctx, ui);
                ui.label(consts::NAME);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    self.draw_console_toggle(ui);
                    ui.separator();
                });
            });
        });

        egui::CentralPanel::default().show(egui_ctx, |ui| {
            self.draw_center(ui, &runtime_data, &mut interface_data);
        });
    }
    
    fn draw_darkmode_toggle(&mut self, egui_ctx: &Context, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            if ui.button("\u{E472}").clicked() {
                self.theme = (self.theme + 1) % self.themes.len();
                mlem_egui_themes::set_theme(egui_ctx, self.get_theme());
            }
        });
    }

    fn draw_console_toggle(&mut self, ui: &mut Ui) {
        let console_updated = self.console.update();

        ui.horizontal(|ui| {
            let button_response = if self.show_console {
                ui.button(format!("{icon} Hide", icon = CONSOLE_ICON))
            } else {
                ui.button(CONSOLE_ICON)
            };
            
            if button_response.clicked() {
                self.show_console = !self.show_console;
            }

            if console_updated {
                button_response.highlight();
            }
        });
    }

    fn draw_about_button(&mut self, ui: &mut Ui) {
        ui.menu_button(format!("v{}", consts::VERSION), |ui| {
            ui.set_max_width(DEFAULT_MENU_WIDTH);    

            ui.menu_button("About", |ui|{
                ui.set_max_width(ABOUT_MENU_WIDTH);
                self.draw_name(ui);
                ui.label(consts::DESCRIPTION);
                ui.separator();

                egui::ScrollArea::vertical().max_height(ABOUT_LICENSE_SCROLL_HEIGHT).show(ui, |ui| {
                    self.draw_info(ui);
                
                    ui.separator();
                    ui.label("Credits");

                    ui.monospace(format!("By {authors}", authors = consts::AUTHORS));
                    ui.separator();
                    ui.monospace(format!("{}", consts::CREDITS));     

                    ui.separator();
                    ui.label("License");
                    ui.monospace(format!("{}", consts::LICENSE_CONTENTS));        
                });
            });
        });
    }

    fn draw_center(&mut self, ui: &mut Ui, runtime_data: &RuntimeData, interface_data: &mut InterfaceData) {
        if self.show_console { 
            self.draw_console(ui, runtime_data, CONSOLE_MAIN_ID);
            return; 
        }

        ui.horizontal(|ui| {
            parameter_label(ui, "Integrated", "Loudness total since reset.",LABEL_WIDTH);

            ui.monospace(format!("{: >5.2} lufs", runtime_data.lufs_global_loudness));
        });

        ui.horizontal(|ui| {
            parameter_label(ui, "Momentary", "Loudness over a duration of 0.4 seconds.", LABEL_WIDTH);

            ui.monospace(format!("{: >5.2} lufs", runtime_data.lufs_momentary_loudness));
            ui.add_space(DEFAULT_SPACE);
            ui.monospace(format!("{: >5.2} db", gain_to_db(1.0 + runtime_data.rms_momentary_loudness)));
        });

        ui.horizontal(|ui| {
            parameter_label(ui, "Short Term", "Loudness over a duration of 3 seconds.", LABEL_WIDTH);

            ui.monospace(format!("{: >5.2} lufs", runtime_data.lufs_shortterm_loudness));
            ui.add_space(DEFAULT_SPACE);
            ui.monospace(format!("{: >5.2} db", gain_to_db(1.0 + runtime_data.rms_shortterm_loudness)));
        });

        ui.horizontal(|ui| {
            parameter_label(ui, "Range", "Loudness range total since reset.", LABEL_WIDTH);

            ui.monospace(format!("{: >5.2} lufs", runtime_data.lufs_range_loudness));
        });

        ui.add_space(ui.available_height() - 12.0);
        ui.horizontal(|ui| {
            let seconds = runtime_data.active_time_ms / 1000.0;
            let minutes = f32::floor(seconds / 60.0);
            
            if ui.button("Reset").clicked() {
                interface_data.reset_meter();
            }
            ui.label(format!("{minutes: >1.0}m{seconds: >1.0}s", minutes = minutes, seconds = seconds - minutes * 60.0));
        });
    }

    fn draw_console(&mut self, ui: &mut Ui, runtime_data: &RuntimeData, hash: impl Hash) {        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let load = (runtime_data.run_ms / (runtime_data.buffer_size as f32 / runtime_data.sample_rate * 1000.0) * 100.0).floor();
                let status = format!("({ms:.2}ms / {load:>3}%) @ {rate}hz, {buff}buf, {channels}ch.", 
                    ms = runtime_data.run_ms,
                    load = load, 
                    rate = runtime_data.sample_rate,
                    buff = runtime_data.buffer_size,
                    channels = runtime_data.channels);
        
                    ui.monospace(format!("{}", status));
            });

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT).with_cross_justify(true), |ui| {
                let log_string = self.console.get_log_string();
                
                egui::ScrollArea::vertical()
                    .id_salt(hash)
                    .show(ui, |ui| {
                        ui.monospace(format!("{}", log_string));
                });
            });
        });
    }

    fn draw_name(&mut self, ui: &mut Ui) {
        ui.heading(format!("{icon} {name}", icon = consts::ICON, name = consts::NAME));
    }

    fn draw_info(&mut self, ui: &mut Ui) {
        ui.label(format!("v{version} {profile} ({id})", version = consts::VERSION, profile = consts::BUILD_TYPE, id = consts::BUILD_ID));
        ui.horizontal(|ui| {
            ui.label("By");
            ui.hyperlink_to(consts::PLUGIN_VENDOR, consts::HOMEPAGE);
        });
    }

    fn get_theme(&self) -> Theme {
        return self.themes[self.theme];
    }
}