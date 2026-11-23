#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

fn main() -> eframe::Result {
    let app_name = "app_name";
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        app_name,
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    msg: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            msg: "Hello World!!".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!("メッセージ：{}", self.msg));
        });
    }
}
