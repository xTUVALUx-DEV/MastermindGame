use egui::{Color32, Grid, Painter, Pos2, Rect, Response, Sense, Ui, Widget};
use mastermind::mastermindlib::board::{BoardSettings, GameState, Guess, MastermindBoard};
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
impl From<&GuessState> for Guess {
    fn from(state: &GuessState) -> Guess {
        Guess {
            0: state.0.iter().map(|&x| x as u8).collect(),
        }
    }
}

impl GuessState {
    pub fn new(settings: &BoardSettings) -> Self {
        Self {
            0: vec![0; settings.code_length as usize],
        }
    }
}

pub struct MastermindWidget<'a> {
    pub board: &'a MastermindBoard,
    pub guess_state: &'a mut GuessState,
}

impl Widget for MastermindWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        //let desired_size = ui.spacing().interact_size.y * 1.5; // example size

        const GRID_SIZE: f32 = 40.0;
        const GRID_SPACING_Y: f32 = 10.0;

        let height = (GRID_SIZE + GRID_SPACING_Y) * (self.board.settings.max_tries + 2) as f32;
        let (rect, response) = ui.allocate_exact_size(egui::vec2(500., height), Sense::click());
        let mut grid_ui = ui.child_ui(rect, *ui.layout(), None);

        egui::Grid::new("mastermind_grid")
            .spacing([10.0, GRID_SPACING_Y])
            .show(&mut grid_ui, |ui| {
                for row in 0..self.board.settings.max_tries {
                    for col in 0..self.board.settings.code_length {
                        let (rect, response) = ui.allocate_exact_size(
                            egui::Vec2::splat(GRID_SIZE),
                            egui::Sense::hover(),
                        );

                        let color = match (self.board.state.guesses.get(row as usize)) {
                            None => egui::Color32::from_rgb(20, 20, 20),
                            Some(guess) => self.board.settings.colors
                                [guess.0[col as usize] as usize]
                                .to_egui_color(),
                        };

                        ui.painter()
                            .circle_filled(rect.center(), rect.width() / 2.0, color);
                    }

                    let answer_opt = self.board.state.answers.get(row as usize);

                    let mut markers = Vec::new();

                    if let Some(answer) = answer_opt {
                        if let GameState::GuessAnswer(right_pos, right_not_pos) = answer {
                            markers.append(&mut vec![1; *right_pos as usize]);
                            markers.append(&mut vec![2; *right_not_pos as usize]);
                        }
                    }

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            for i in 0..2 {
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::Vec2::splat(GRID_SIZE / 2.0),
                                    egui::Sense::hover(),
                                );
                                if markers.get(i).is_some() {
                                    ui.painter().circle_filled(
                                        rect.center(),
                                        rect.width() / 2.0,
                                        match (markers.get(i)) {
                                            Some(1) => egui::Color32::from_rgb(255, 255, 255),
                                            Some(2) => egui::Color32::from_rgb(26, 166, 150),
                                            _ => egui::Color32::from_rgb(255, 0, 0),
                                        },
                                    );
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            for i in 2..4 {
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::Vec2::splat(GRID_SIZE / 2.0),
                                    egui::Sense::hover(),
                                );
                                if markers.get(i).is_some() {
                                    ui.painter().circle_filled(
                                        rect.center(),
                                        rect.width() / 2.0,
                                        match (markers.get(i)) {
                                            Some(1) => egui::Color32::from_rgb(255, 255, 255),
                                            Some(2) => egui::Color32::from_rgb(26, 166, 150),
                                            _ => egui::Color32::from_rgb(255, 0, 0),
                                        },
                                    );
                                }
                            }
                        });
                    });

                    ui.end_row();
                }

                // Guess
                for col in 0..self.board.settings.code_length {
                    let (rect, response) =
                        ui.allocate_exact_size(egui::Vec2::splat(40.0), egui::Sense::click());
                    if (response.clicked()) {
                        self.guess_state.0[col as usize] += 1;
                        self.guess_state.0[col as usize] %= self.board.settings.colors.len() as i16;
                    }
                    ui.painter().circle_filled(
                        rect.center(),
                        rect.width() / 2.0,
                        self.board.settings.colors[self.guess_state.0[col as usize] as usize]
                            .to_egui_color(),
                    );
                }
            });
        response
    }
}
