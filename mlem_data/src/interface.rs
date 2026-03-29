use std::{io::Error, str::FromStr, sync::{Arc, Mutex, atomic::Ordering}};
use egui_file::FileDialog;
use mlem_base::{base::mlem_interface::MlemInterface, interface::{self, param_combo_box, param_drag_value::ParamDragValue, param_toggle::{self, ParamToggle}, utils::{parameter_grid, parameter_label}}};
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{Align, Context, Layout, Ui, Vec2};
use crate::{consts::{self, PLUGIN_METADATA}, params::{DATA_PREVIEW_SIZE_FULL, DATA_PREVIEW_SIZE_SMALL, DataParams, MAX_MONOSPACE_WIDTH}};

pub struct MeterInterface {
    params: Arc<DataParams>,
    open_file_dialog: Mutex<FileDialog>,
    select_folder_dialog: Mutex<FileDialog>
}

impl MeterInterface {
    pub fn new(params: Arc<DataParams>) -> Self {
        return Self {
            params,
            open_file_dialog: Mutex::from(FileDialog::open_file(None)
                .resizable(false)
                .default_size(Vec2::new(PLUGIN_METADATA.window_width  as f32 - interface::DEFAULT_SPACE * 8.0, PLUGIN_METADATA.window_height as f32 / 2.0))
                .show_rename(false)),
            select_folder_dialog: Mutex::from(FileDialog::select_folder(None)
                .resizable(false)
                .default_size(Vec2::new(PLUGIN_METADATA.window_width  as f32 - interface::DEFAULT_SPACE * 8.0, PLUGIN_METADATA.window_height as f32 / 2.0))
                .show_rename(false))
        };
    }

        fn bar_mute(&self, ui: &mut Ui, setter: &ParamSetter) {
        ui.add(param_toggle::ParamToggle::for_param(&self.params.mute, setter, "Mute", "Mute").fill(false));
    }

    fn bar_file(&self, ui: &mut Ui, ctx: &Context) {
        let mut open_file_dialog = self.open_file_dialog.lock().unwrap();
        let select_folder_dialog = self.select_folder_dialog.lock().unwrap();

        ui.add_enabled_ui(!open_file_dialog.visible() && !select_folder_dialog.visible(), |ui| {
            if (ui.button("File")).clicked() {
                open_file_dialog.open();
            }
        });


        if open_file_dialog.show(ctx).selected() {
            if let Some(path) = open_file_dialog.path() {
                let Ok(path) = String::from_str(&path.to_string_lossy());
                let mut paths = self.params.paths.lock().unwrap();
                paths.clear();
                paths.push(path);
                self.params.path_refresh.store(true, Ordering::Relaxed);
                self.params.path_current.store(paths.len(), Ordering::Relaxed);
            }
        }
    }

    fn bar_folder(&self, ui: &mut Ui, ctx: &Context) {
        let mut select_folder_dialog = self.select_folder_dialog.lock().unwrap();
        let open_file_dialog = self.open_file_dialog.lock().unwrap();

        ui.add_enabled_ui(!open_file_dialog.visible() && !select_folder_dialog.visible(), |ui| {
            if (ui.button("Folder")).clicked() {
                select_folder_dialog.open();
            }
        });

        if select_folder_dialog.show(ctx).selected() {
            if let Some(path) = select_folder_dialog.path() {
                let mut paths = self.params.paths.lock().unwrap();
                let Ok(path_lossy) = String::from_str(&path.to_string_lossy());
                if let Ok(_) = Self::update_filepaths_from_folder(path_lossy, &mut paths) {
                    self.params.path_refresh.store(true, Ordering::Relaxed);
                    self.params.path_current.store(paths.len(), Ordering::Relaxed);
                }
            }
        }
    }

    fn update_filepaths_from_folder(path: String, paths: &mut Vec<String>) -> Result<(), Error> {
        paths.clear();

        let Ok(path) = String::from_str(&path);
        if let Ok(directory) = std::fs::read_dir(path) {
            for file in directory {
                if let Ok(file) = file {
                    let Ok(path) = String::from_str(&file.path().to_string_lossy());
                    paths.push(path);
                } 
            }
        }

        return Ok(());
    }

    fn build_data_string(&self, length: usize) -> String {
        let mut data_string = String::new();
        let data_preview = *self.params.data_preview.lock().unwrap();
        for i in 0..length {
            data_string.push_str(format!("{:02X?}", data_preview[i]).as_str());
        }


        let paths = self.params.paths.lock().unwrap();
        let path_current = self.params.path_current.load(Ordering::Relaxed);
        if  path_current < paths.len() {
            let mut path = paths[path_current].clone();
            path.truncate(MAX_MONOSPACE_WIDTH - 7);

            for _ in 0..path.len() {
                data_string.pop();
            }

            data_string.pop();
            data_string.pop();
            data_string.pop();
            data_string.pop();

            data_string.pop();
            data_string.push(' ');
            data_string.push_str(&path);

            data_string.push(' ');
            data_string.push_str(&format!("{:02}%", f32::floor(self.params.data_progress.load(Ordering::Relaxed) * 100.0)));
        }

        return data_string;
    }
}

impl MlemInterface<DataParams> for MeterInterface {
    fn params(&self) ->  Arc<DataParams> {
        return self.params.clone();
    }

    fn build(&mut self, ctx: &Context) { }
    
    fn update_bar(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter) {
        self.bar_mute(ui, setter);
        self.bar_file(ui, ctx);
        self.bar_folder(ui, ctx);
    }
    
    fn update_center(&mut self, ui: &mut Ui, ctx: &Context, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.add(param_toggle::ParamToggle::for_param(&self.params.mono, setter, "Mono", "Stereo").fill(false));
            ui.add(param_combo_box::ParamComboBox::for_param(&self.params.read_mode, setter).fill(false));
        });
        
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            if self.params.mute.value() {
                ui.monospace(consts::DISCLAIMER);
                ui.separator();
            }

            ui.add_enabled_ui(!self.params.mute.value(), |ui| {
                let data_string = self.build_data_string(if self.params.mute.value() { DATA_PREVIEW_SIZE_SMALL } else { DATA_PREVIEW_SIZE_FULL });

                ui.monospace(data_string);
            });
            ui.separator();
        });
    }
}