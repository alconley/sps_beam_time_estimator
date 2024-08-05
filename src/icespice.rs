#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ICESPICERunTimeSettings {
    pub n_particle_counts: i64,
    pub transmission_prob: f64,   // in percentage
    pub detector_efficiency: f64, // in percentage
    pub branching_ratio: f64,     // in percentage
    pub conversion_coefficient: f64,
}

impl Default for ICESPICERunTimeSettings {
    fn default() -> Self {
        Self {
            n_particle_counts: 10000,
            transmission_prob: 1.29,
            detector_efficiency: 30.7,
            branching_ratio: 100.0,
            conversion_coefficient: 1.0,
        }
    }
}

impl ICESPICERunTimeSettings {
    // Method to calculate the number of conversion electrons
    pub fn calculate_conversion_electrons(&self) -> f64 {
        let gamma_rays = self.n_particle_counts as f64 * (self.branching_ratio / 100.0);
        let conversion_electrons = gamma_rays * self.conversion_coefficient;
        conversion_electrons * (self.transmission_prob / 100.0) * (self.detector_efficiency / 100.0)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("icespice_runtime_settings_grid")
            .striped(true)
            .show(ui, |ui| {
                ui.heading("ICESPICE");
                ui.end_row();

                ui.label("Particle Counts:")
                    .on_hover_text("Number of particles detected in the excited state.");
                ui.add(
                    egui::DragValue::new(&mut self.n_particle_counts)
                        .speed(1.0)
                        .range(0..=i64::MAX),
                );
                ui.end_row();

                ui.label("Transmission Probability:").on_hover_text(
                    "Probability of the particle passing through ICESPICE to the detector in 4π.",
                );
                ui.add(egui::DragValue::new(&mut self.transmission_prob)
                    .suffix(" %")
                    .speed(0.1)
                    .range(0.0..=100.0),
                );
                ui.end_row();

                ui.label("Detector Efficiency:").on_hover_text(
                    "Efficiency of the detector for an electron to deposit its full energy.",
                );
                ui.add(
                    egui::DragValue::new(&mut self.detector_efficiency)
                    .suffix(" %")
                    .speed(0.1)
                    .range(0.0..=100.0),
                );
                ui.end_row();

                ui.label("Branching Ratio:").on_hover_text("The branching ratio is the fraction of decays that proceed through a particular transition.");
                ui.add(
                    egui::DragValue::new(&mut self.branching_ratio)
                    .suffix(" %")
                    .speed(1.0)
                    .range(0.0..=100.0),
                );
                ui.end_row();

                ui.label("Conversion Coefficient (α):").on_hover_text("The internal conversion coefficient (α) is the ratio of the number of conversion electrons emitted to the number of gamma rays emitted for a particular transition. ICCs depend on the energy of the transition and the atomic number of the nucleus. These coefficients can be obtained from tables or calculated using theoretical models.");
                ui.add(egui::DragValue::new(&mut self.conversion_coefficient));
                ui.end_row();

                let conversion_electrons = self.calculate_conversion_electrons();
                ui.label("Estimated Number of\nDetected Conversion Electrons:")
                .on_hover_text("Formula: Particle Counts * (Branching Ratio [%] / 100) * α * (Transmission Probability [%] / 100) * (Detector Efficiency [%] / 100)");
                ui.label(format!("{:.0}", conversion_electrons));
                ui.end_row();
            });
    }
}
