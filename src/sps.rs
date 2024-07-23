use eframe::egui::{self};

const CHARGE: f64 = 1.6e-19; // Elementary charge in C
const NA: f64 = 6.023e23; // Avogadro's number

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SPSRunTimeSettings {
    cross_section: f64,     // µb/sr
    target_density: f64,    // µg/cm^2
    target_molar_mass: f64, // g/mol
    beam_current: f64,      // nA
    z_beam: i32,            // proton number
    slit_settings: f64,     // msr
    desired_counts: i64,    // counts
    time_s: f64,            // seconds
    time_h: f64,            // hours
    time_d: f64,            // days
}

impl Default for SPSRunTimeSettings {
    fn default() -> Self {
        Self {
            cross_section: 100.0,
            target_density: 100.0,
            target_molar_mass: 240.0,
            beam_current: 20.0,
            z_beam: 1,
            slit_settings: 4.62,
            desired_counts: 1000,
            time_s: 0.0,
            time_h: 0.0,
            time_d: 0.0,
        }
    }
}

impl SPSRunTimeSettings {
    fn calculate_beam_time(&mut self) {
        let slits_sr = self.slit_settings * 1e-3; // msr to sr
        let target_density = self.target_density * 1e-6; // µg/cm^2 to g/cm^2
        let beam_current = self.beam_current * 1e-9; // nA to A
        let f_target = (target_density * NA) / (self.target_molar_mass) * (1e-24) * (1e-6);

        let run_time_s = (self.z_beam as f64 * CHARGE * self.desired_counts as f64)
            / (self.cross_section * f_target * slits_sr * beam_current);

        self.time_s = run_time_s;
        self.time_h = run_time_s / 3600.0;
        self.time_d = self.time_h / 24.0;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("sps_runtime_settings_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.heading("SE-SPS");
                ui.end_row();

                ui.label("Cross Section:");

                    // Display and adjust the cross section as microbarns
                ui.add(
                    egui::DragValue::new(&mut self.cross_section)
                        .speed(1.0)
                        .suffix(" µb/sr")
                        .range(0.0..=f64::INFINITY)
                    );
                ui.end_row();


                ui.label("Target Density:");
                ui.add(
                    egui::DragValue::new(&mut self.target_density)
                        .speed(1.0)
                        .suffix(" µg/cm^2")
                        .range(0.0..=f64::INFINITY)
                );
                ui.end_row();

                ui.label("Target Molar Mass:");
                ui.add(
                    egui::DragValue::new(&mut self.target_molar_mass)
                        .speed(1.0)
                        .suffix(" g/mol")
                        .range(0.0..=f64::INFINITY)
                );
                ui.end_row();

                ui.label("Beam Current:");
                ui.add(
                    egui::DragValue::new(&mut self.beam_current)
                        .speed(1.0)
                        .suffix(" nA")
                        .range(0.0..=f64::INFINITY)
                ).on_hover_text("Beam current on target.");
                ui.end_row();

                ui.label("Z Beam:");
                ui.add(
                    egui::DragValue::new(&mut self.z_beam)
                        .speed(1.0)
                        .prefix("Z = ")
                        .range(1..=118) // Adjusted to allow minimum 1
                ).on_hover_text("Proton number of the beam.");
                ui.end_row();

                ui.label("Slit Settings:");
                ui.add(
                    egui::DragValue::new(&mut self.slit_settings)
                        .speed(0.1)
                        .suffix(" msr")
                        .range(0.0..=12.8)
                ).on_hover_text("Solid angle of the SE-SPS. Typical value is 4.62 msr. The SE-SPS has a max solid angle of 12.8 msr.");
                ui.end_row();

                ui.label("Counts:");
                ui.add(
                    egui::DragValue::new(&mut self.desired_counts)
                        .speed(1.0)
                        .suffix(" counts")
                        .range(0..=i64::MAX)
                ).on_hover_text("The desired number of counts in the peak of interest.");
                ui.end_row();

                // Call calculate_beam_time here if it should happen automatically upon any change
                self.calculate_beam_time();
                ui.label("Estimated Time:");
                ui.label(format!("{:.0} s | {:.2} h | {:.2} d", self.time_s, self.time_h, self.time_d));
                ui.end_row();
        });
    }
}
