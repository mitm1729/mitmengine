mod gameobjects;

use gameobjects::board::Board;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let app = Board::new();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 800.0)), 
        ..Default::default()
    };

    eframe::run_native("Chess Game", options, Box::new(|_| Box::new(app)))
}
