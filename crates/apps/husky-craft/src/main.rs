use eframe::egui;
use husky_code_edit::CodeEdit;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Husky Notebook",
        options,
        Box::new(|_cc| Box::new(HuskyCraft::default())),
    )
}

struct HuskyCraft {
    name: String,
    age: u32,
}

impl Default for HuskyCraft {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: Default::default(),
        }
    }
}

impl eframe::App for HuskyCraft {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        use eframe::egui::Widget;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Husky Notebook");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                // ui.text_edit_singleline(&mut self.name)
                //     .labelled_by(name_label.id);
                CodeEdit::singleline(&mut self.name).ui(ui);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.label(format!(
                "Formula: {}xdx",
                husky_unicode_symbols::opr::OPR_INTEGRAL
            ))
        });
    }
}