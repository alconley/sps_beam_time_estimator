use super::cebra::CeBrARunTimeSettings;
use super::icespice::ICESPICERunTimeSettings;
use super::sps::SPSRunTimeSettings;
use eframe::egui::{self};
use eframe::App;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BeamTimeApp {
    sps_settings: SPSRunTimeSettings,
    cebra_settings: CeBrARunTimeSettings,
    icespice_settings: ICESPICERunTimeSettings,
    show_sps: bool,
    show_cebra: bool,
    show_icespice: bool,
    window: bool,
}

impl Default for BeamTimeApp {
    fn default() -> Self {
        Self {
            sps_settings: SPSRunTimeSettings::default(),
            cebra_settings: CeBrARunTimeSettings::default(),
            icespice_settings: ICESPICERunTimeSettings::default(),
            show_sps: true,
            show_cebra: true,
            show_icespice: true,
            window: false,
        }
    }
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
        egui::menu::bar(ui, |ui| {
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut self.show_sps, "Show SPS Estimator");
                ui.checkbox(&mut self.show_cebra, "Show CeBrA Estimator");
                ui.checkbox(&mut self.show_icespice, "Show ICESPICE Estimator");
            });
        });

        egui::SidePanel::left("sps_panel")
            .resizable(false)
            .show_animated_inside(ui, self.show_sps, |ui| {
                self.sps_settings.ui(ui);
            });

        egui::SidePanel::left("cebra_panel")
            .resizable(false)
            .show_animated_inside(ui, self.show_cebra, |ui| {
                self.cebra_settings.ui(ui);
            });

        egui::SidePanel::left("icespice_panel")
            .resizable(false)
            .show_animated_inside(ui, self.show_icespice, |ui| {
                self.icespice_settings.ui(ui);
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
