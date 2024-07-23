#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Efficiency {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Efficiency {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { a, b, c, d }
    }

    pub fn calculate_efficiency(&self, energy: f64) -> f64 {
        self.a * (-energy / self.b).exp() + self.c * (-energy / self.d).exp()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::DragValue::new(&mut self.a)
                .prefix("a = ")
                .speed(0.1)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
        ui.add(
            egui::DragValue::new(&mut self.b)
                .prefix("b = ")
                .speed(10.0)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
        ui.add(
            egui::DragValue::new(&mut self.c)
                .prefix("c = ")
                .speed(0.1)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
        ui.add(
            egui::DragValue::new(&mut self.d)
                .prefix("d = ")
                .speed(10.0)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Detector {
    pub name: String,
    pub efficiency: Efficiency,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Decay {
    pub energy: f64,
    pub absolute_intensity: f64,
    pub efficiency: f64,
    pub efficiency_corrected_counts: f64,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CeBrARunTimeSettings {
    n_particle_counts: i64,
    decay: Decay,
}

impl CeBrARunTimeSettings {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("cebra_runtime_settings_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.heading("CeBrA");
                ui.end_row();
                ui.label("Particle Counts:")
                    .on_hover_text("Number of particles detected in the excited state");
                ui.add(
                    egui::DragValue::new(&mut self.n_particle_counts)
                        .speed(1.0)
                        .range(0..=i64::MAX),
                );
                ui.end_row();

                ui.label("γ Decay");
                ui.label("Energy");
                ui.label("Intensity")
                    .on_hover_text("Absolute intensity of the decay");
                ui.end_row();
                ui.label("");
                ui.add(
                    egui::DragValue::new(&mut self.decay.energy)
                        .suffix(" keV")
                        .speed(1.0)
                        .range(0.0..=f64::INFINITY),
                );
                ui.add(
                    egui::DragValue::new(&mut self.decay.absolute_intensity)
                        .suffix(" %")
                        .speed(0.1)
                        .range(0.0..=f64::INFINITY),
                );

                ui.end_row();

                ui.label(format!("Efficiency at\n{} keV: ", self.decay.energy));
                ui.add(
                    egui::DragValue::new(&mut self.decay.efficiency)
                        .suffix(" %")
                        .speed(0.1)
                        .range(0.0..=100.0),
                );

                ui.end_row();

                ui.label("Estimated Counts:").on_hover_text(
                    "Estimated counts = Particle counts * Absolute intensity * Efficiency",
                );

                let estimated_counts =
                    self.n_particle_counts as f64 * self.decay.absolute_intensity / 100.0
                        * self.decay.efficiency
                        / 100.0;

                ui.label(format!("{:.2}", estimated_counts));

                ui.end_row();
            });
    }
}
