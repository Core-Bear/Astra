use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use egui::{Grid, Ui};

use crate::{AppConfig, Theme};

pub fn config_editor(ui: &mut Ui, config: &mut AppConfig) {
    Grid::new("config_editor").num_columns(2).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.label("Theme");
        });
        ui.vertical(|ui| {
            if ui
                .radio_value(&mut config.theme, Theme::Latte, "Latte")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), LATTE);
            }
            if ui
                .radio_value(&mut config.theme, Theme::Frappe, "Frappé")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), FRAPPE);
            }
            if ui
                .radio_value(&mut config.theme, Theme::Macchiato, "Macchiato")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), MACCHIATO);
            }
            if ui
                .radio_value(&mut config.theme, Theme::Mocha, "Mocha")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), MOCHA);
            }
        });
        ui.end_row();

        ui.label("Script Editor Process");
        ui.text_edit_singleline(&mut config.script_editor_process);
        ui.end_row();

        ui.label("Script Editor Command Args");
        ui.text_edit_singleline(&mut config.script_editor_command_args);
        ui.end_row();
    });
}
