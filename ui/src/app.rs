use egui::Widget;
use mastermind::mastermindlib::board::{BoardSettings, Guess, MastermindBoard, GameState};
use rgb::RGB8;

use crate::mastermindwidget::{GuessState, MastermindWidget};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    settings: BoardSettings,
    color_count: i16,

    #[serde(skip)]
    show_win: bool,

    #[serde(skip)]
    show_loss: bool,

    #[serde(skip)]
    guess_state: GuessState,

    #[serde(skip)]
    board: MastermindBoard,

    #[serde(skip)]
    current_page: Page,
}

enum Page {
    Home,
    Settings,
}

impl Default for App {
    fn default() -> Self {
        let mut settings = BoardSettings::default().code_length(4).max_tries(8);
        settings.generate_colors(4);
        Self {
            guess_state: GuessState::new(&settings),
            color_count: settings.colors.len() as i16,
            settings: settings.clone(),
            board: MastermindBoard::new(settings),
            current_page: Page::Home,
            show_loss: false,
            show_win: false,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: App = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        app.board = MastermindBoard::new(app.settings.clone());
        app.guess_state = GuessState::new(&app.settings);

        app
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.horizontal(|ui| {
                    if ui.button("Home").clicked() {
                        self.current_page = Page::Home;
                    }

                    if ui.button("Settings").clicked() {
                        self.current_page = Page::Settings;
                    }

                    if ui.button("Reset").clicked() {
                        self.board = MastermindBoard::new(self.settings.clone());
                        self.guess_state = GuessState::new(&self.settings);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.current_page {
            Page::Home => {
                ui.heading("Mastermind");
                MastermindWidget {
                    board: &self.board,
                    guess_state: &mut self.guess_state,
                }
                .ui(ui);
                if ui.button("Guess").clicked() {
                    if let GameState::GameEnd(has_won) = self.board.guess(&Guess::from(&self.guess_state)) {
                        if (has_won) {
                            self.show_win = true;
                        } else {
                            self.show_loss = true;
                        }
                    }
                }
            }
            Page::Settings => {
                ui.heading("Settings");
                ui.add(
                    egui::Slider::new(&mut self.settings.code_length, 1..=10).text("Code Length"),
                );

                ui.add(egui::Slider::new(&mut self.settings.max_tries, 1..=12).text("Max Tries"));

                let response =
                    ui.add(egui::Slider::new(&mut self.color_count, 2..=9).text("Color Count"));
                if response.changed() {
                    self.settings.generate_colors(self.color_count);
                }

                ui.separator();
            }
        });

        // Modals
        if self.show_win {
            egui::Modal::new("win_modal".into()).show(ctx, |ui| {
                ui.label("You won");
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        self.show_win = false;
                    }
                });
            });
        }

        if self.show_loss {
            egui::Modal::new("loss_modal".into()).show(ctx, |ui| {
                ui.label("You're out of moves");
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        self.show_loss = false;
                    }
                });
            });
        }
    }
}
