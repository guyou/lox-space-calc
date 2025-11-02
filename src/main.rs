use eframe::egui;

use lox_time::time::Time;
use lox_time::time_scales::TimeScale;
use lox_time::time_scales::DynTimeScale;
use lox_time::time_scales::{Tai, Tcb, Tcg, Tdb, Tt, Ut1};
use lox_time::TimeError::TimeError;

mod times_ui;

#[derive(Default)]
struct ExampleApp {
    input_scale: DynTimeScale,
    input_time: String,
    output_scale: DynTimeScale,
    output_time: String,
}

impl ExampleApp {
    fn name() -> &'static str {
        "lox-calc"
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
            ui.label("Time Scale:");

            times_ui::time_scale_combo(ui, "Input Scale", &mut self.input_scale);
            ui.label("Time:");
            ui.text_edit_singleline(&mut self.input_time);

            times_ui::time_scale_combo(ui, "Output Scale", &mut self.output_scale);

            let input = Time::from_iso(self.input_scale, &self.input_time.as_str());
            //let input = self.input_scale self.input_time.parse();
            let output = match input {
                Ok(input) => input.with_scale(self.output_scale).to_string(),
                Err(err) => "Err".to_string()
            };

            ui.label("Time:");
            //ui.text_edit_singleline(&mut self.output_time);
            ui.label(output);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
