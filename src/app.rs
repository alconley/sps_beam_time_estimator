use super::cebra::CeBrARunTimeSettings;
use super::sps::SPSRunTimeSettings;
use eframe::egui::{self};
use eframe::App;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct BeamTimeApp {
    sps_settings: SPSRunTimeSettings,
    cebra_settings: CeBrARunTimeSettings,
    window: bool,
}

impl BeamTimeApp {
    pub fn new(cc: &eframe::CreationContext<'_>, window: bool) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self {
            window,
            ..Default::default()
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("sps_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                self.sps_settings.ui(ui);
            });
        egui::SidePanel::left("cebra_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                self.cebra_settings.ui(ui);
            });
    }
}

impl App for BeamTimeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.window {
            egui::Window::new("Beam Time Estimator").show(ctx, |ui| {
                self.ui(ui);
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.ui(ui);
            });
        }
    }
}
