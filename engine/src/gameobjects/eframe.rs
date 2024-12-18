use eframe::egui::{CentralPanel, Color32, TopBottomPanel};

use crate::gameobjects::{
    board::Board,
    piece::Color,
};

/**
 * This is used to add visual configuration of the board data to the eframe
 */
impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let top_panel_response = TopBottomPanel::top("my_top_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(5.0); // Add a padding of 5
                ui.centered_and_justified(|ui| {
                    ui.heading("Rust Chess Engine");
                });
            });
        });
        
        let bottom_panel_response = TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("Powered by egui");
            });
        });

        // Calculate the height of the top and bottom panels
        let top_panel_height = top_panel_response.response.rect.height() + 50.0;
        let bottom_panel_height = bottom_panel_response.response.rect.height();

        // let top_panel_height = 0.0;
        // let bottom_panel_height = 0.0;
        
        CentralPanel::default().show(ctx, |ui| {
            // Calculate available space for the chessboard
            let available_width = ui.available_width();
            let available_height = ui.available_height();
        
            // Deduct the panel heights from the available height
            let usable_height = available_height - top_panel_height - bottom_panel_height;
        
            // Calculate cell dimensions based on the smaller of width or usable height
            let cell_size = available_width.min(usable_height) / 8.0;
        
            // Center the chessboard in the remaining usable space
            let board_width = cell_size * 8.0;
            let board_height = cell_size * 8.0;
        
            let x_offset = (available_width - board_width) / 2.0;
            let y_offset = (usable_height - board_height) / 2.0 + top_panel_height;
        
            // Use a painter to handle drawing
            let painter = ui.painter();
        
            // First draw the tiles 
            for (i, pieces_row) in self.board.iter().enumerate() {
                for (j, piece) in pieces_row.iter().enumerate() {
                    // Determine tile background color
                    let background_color = if (i + j) % 2 == 0 {
                        Color32::from_rgb(240, 217, 181) // Light square color
                    } else {
                        Color32::from_rgb(181, 136, 99) // Dark square color
                    };

                    // Calculate the position for the current tile
                    let x = x_offset + j as f32 * cell_size;
                    let y = y_offset + i as f32 * cell_size;
                    
                    let text_x = x + cell_size / 2.0;
                    let text_y = y + cell_size / 2.0;
                    

                    // Define the rectangle for the current tile
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(cell_size, cell_size),
                    );
        
                    // Draw the rectangle for the tile
                    painter.rect_filled(rect, 0.0, background_color);
                    
                    // Draw initial position of chess pieces
                    if let Some(piece) = piece {
                        // Determine piece Unicode and color
                        let piece_unicode = piece.to_unicode();
                        let piece_color = match piece.color {
                            Color::White => Color32::WHITE,
                            Color::Black => Color32::BLACK,
                        };
                        
                        painter.text(
                            egui::pos2(text_x, text_y),
                            egui::Align2::CENTER_CENTER,
                            piece_unicode,
                            egui::FontId::proportional(cell_size * 0.6), // Scale font size
                            piece_color,
                        );
                    }
                }
            }
        });  
    }
}
