use egui::{Color32, Painter, Grid, Rect, Response, Sense, Ui, Widget, Pos2};
use mastermind::mastermindlib::board::{BoardSettings, MastermindBoard};
use rgb::RGB8;

trait egui_color_convertable {
    fn to_egui_color(&self) -> egui::Color32;
}

impl egui_color_convertable for RGB8 {
    fn to_egui_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
    }
}

pub struct GuessState(Vec<i16>);

impl GuessState {
    pub fn new(settings: &BoardSettings) -> Self {
        Self {
            0: vec![0; settings.code_length as usize]
        }

    }
}

pub struct MastermindWidget<'a> { 
    pub board: &'a MastermindBoard,
    pub guess_state: &'a mut GuessState
}

impl Widget for MastermindWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {

        let desired_size = ui.spacing().interact_size.y * 1.5; // example size
                                                               //
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(500., 700.),
            Sense::click(),
        );
        let mut grid_ui = ui.child_ui(rect, *ui.layout(), None);

        egui::Grid::new("mastermind_grid")
            .spacing([10.0, 10.0])
            .show(&mut grid_ui, |ui| {
                for row in 0..self.board.settings.max_tries {
                    for col in 0..self.board.settings.code_length {
                        let (rect, response) = ui.allocate_exact_size(
                            egui::Vec2::splat(40.0),
                            egui::Sense::hover(), // optional
                        );

                        // draw the circle in the cell
                        ui.painter().circle_filled(
                            rect.center(),
                            rect.width() / 2.0,
                            egui::Color32::from_rgb(200, 50, 50),
                        );
                    }
                    ui.end_row(); // move to next row
                }

            // Guess
                for col in 0..self.board.settings.code_length {
                    let (rect, response) = ui.allocate_exact_size(
                        egui::Vec2::splat(40.0),
                        egui::Sense::click(),
                    );
                    if (response.clicked()) {
                        println!("Clicked!");
                        self.guess_state.0[col as usize] += 1;
                        self.guess_state.0[col as usize] %= self.board.settings.colors.len() as i16;
                    }
                    ui.painter().circle_filled(
                        rect.center(),
                        rect.width() / 2.0,
                        self.board.settings.colors[
                            self.guess_state.0[col as usize] as usize
                        ].to_egui_color(),
                    );
                }
            });
        response
    }
}
