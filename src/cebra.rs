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
                // .prefix("a = ")
                .speed(0.1)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
        ui.add(
            egui::DragValue::new(&mut self.b)
                // .prefix("b = ")
                .speed(10.0)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
        ui.add(
            egui::DragValue::new(&mut self.c)
                // .prefix("c = ")
                .speed(0.1)
                .range(0.0..=f64::INFINITY),
        )
        .on_hover_text("Efficiency = a * exp(-energy / b) + c * exp(-energy / d)");
        ui.add(
            egui::DragValue::new(&mut self.d)
                // .prefix("d = ")
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Decay {
    pub energy: f64,
    pub absolute_intensity: f64,
    pub efficiency: f64,
    pub efficiency_corrected_counts: f64,
}

impl Default for Decay {
    fn default() -> Self {
        Self {
            energy: 2000.0,
            absolute_intensity: 100.0,
            efficiency: 0.0,
            efficiency_corrected_counts: 0.0,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CeBrARunTimeSettings {
    n_particle_counts: i64,
    decay: Decay,
    detectors: Vec<Detector>,
}

impl Default for CeBrARunTimeSettings {
    fn default() -> Self {
        Self {
            n_particle_counts: 10000,
            decay: Decay::default(),
            detectors: vec![],
        }
    }
}

impl CeBrARunTimeSettings {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("cebra_runtime_settings_grid")
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
                        .speed(10.0)
                        .range(0.0..=f64::INFINITY),
                );
                ui.add(
                    egui::DragValue::new(&mut self.decay.absolute_intensity)
                        .suffix(" %")
                        .speed(0.1)
                        .range(0.0..=100.0),
                );

                ui.end_row();

                ui.heading("Detectors");

                if ui.button("+").clicked() {
                    self.detectors.push(Detector {
                        name: format!("Detector {}", self.detectors.len() + 1),
                        efficiency: Efficiency::default(),
                    });
                }

                if ui.button("REU-2023").on_hover_text("Efficiency values for the 52Cr(d,pγ)53Cr and 34S(d,pγ)35S experiments that took place in the summer of 2023 during the REU.").clicked() {
                    self.detectors.clear();
                    self.detectors.push(Detector {
                        name: "Detector 0".to_string(),
                        efficiency: Efficiency::new(1.04342, 313.36388, 0.30550, 2796.19080),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 1".to_string(),
                        efficiency: Efficiency::new(0.91597, 344.80832, 0.26477, 3074.53024),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 2".to_string(),
                        efficiency: Efficiency::new(0.34643, 391.57405, 0.11673, 4392.80188),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 3".to_string(),
                        efficiency: Efficiency::new(0.95401, 292.86782, 0.30357, 2592.23281),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 4".to_string(),
                        efficiency: Efficiency::new(1.69550, 304.59392, 0.93590, 4628.69818),
                    });
                }

                if ui.button("Summer-2022").on_hover_text("Efficiency values for the 49Ti(d,pγ)50Ti and 61Ni(d,pγ)62Ni experiments that took place in the summer of 2022 during CeBrA's commissioning run.").clicked() {
                    self.detectors.clear();
                    self.detectors.push(Detector {
                        name: "Detector 0".to_string(),
                        efficiency: Efficiency::new(3.44699544e-01, 2.49383011e+03, 1.00560372e+00, 3.31802347e+02),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 1".to_string(),
                        efficiency: Efficiency::new(3.31397563e-01, 2.57096172e+03, 1.01446003e+00, 3.45717989e+02),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 2".to_string(),
                        efficiency: Efficiency::new(2.99597105e-01, 2.61859263e+03, 8.69973182e-01, 3.60128895e+02),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 3".to_string(),
                        efficiency: Efficiency::new(3.19707493e-01, 2.37437634e+03, 8.53245327e-01, 3.39245613e+02),
                    });

                    self.detectors.push(Detector {
                        name: "Detector 4".to_string(),
                        efficiency: Efficiency::new(5.22246571e-01, 3.76416626e+03, 7.08680303e-01, 4.16212259e+02),
                    });
                }


                ui.end_row();

                ui.label("Name");
                ui.label("a");
                ui.label("b");
                ui.label("c");
                ui.label("d");
                ui.label("ε(γ)");
                ui.label("Counts");
                ui.label("");
                ui.end_row();

                let mut index_to_remove = None;

                let mut total_efficiency = 0.0;
                let mut total_counts = 0.0;
                for (index, detector) in self.detectors.iter_mut().enumerate() {
                    ui.text_edit_singleline(&mut detector.name);
                    detector.efficiency.ui(ui);

                    let efficiency = detector.efficiency.calculate_efficiency(self.decay.energy);
                    ui.label(format!("{:.2} %", efficiency));

                    let expected_counts =
                        self.n_particle_counts as f64 * self.decay.absolute_intensity / 100.0
                            * efficiency
                            / 100.0;

                    ui.label(format!("{:.0}", expected_counts)).on_hover_text(
                        "Estimated counts = Particle counts * Absolute intensity [%] /100 * Efficiency [%] /100",
                    );

                    if ui.button("-").clicked() {
                        index_to_remove = Some(index);
                    }
                    ui.end_row();

                    total_efficiency += efficiency;
                    total_counts += expected_counts;

                }

                if let Some(index) = index_to_remove {
                    self.detectors.remove(index);
                }

                ui.label("");
                ui.label("");
                ui.label("");
                ui.label("");
                ui.label("Total");
                ui.label(format!("{:.2} %", total_efficiency));
                ui.label(format!("{:.0}", total_counts));
                ui.end_row();

            });
    }
}
