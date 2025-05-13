use nih_plug_egui::egui::{self, RichText, Ui, Vec2, WidgetText};

pub const TOOLTIP_HOVER_WIDTH: f32 = 300.0;

pub fn help_label(ui: &mut Ui, text: impl Into<RichText>) {    
    ui.add_enabled_ui(false, |ui| {
        ui.label("\u{E3E8}").on_disabled_hover_ui(|ui| {
            ui.set_max_width(TOOLTIP_HOVER_WIDTH);
            ui.monospace(text);
        });
    });
}

pub fn toggle_value(ui: &mut Ui, value: &mut bool, true_text: impl Into<WidgetText>, false_text: impl Into<WidgetText>, size: impl Into<Vec2>) {
    if *value {
        if ui.add_sized(size, egui::SelectableLabel::new(*value, true_text)).clicked() {
            *value = !*value;
        }
    } else {
        if ui.add_sized(size, egui::SelectableLabel::new(*value, false_text)).clicked() {
            *value = !*value;
        }
    }
}