use mastermind::mastermindlib::board::{BoardSettings, MastermindBoard};
use rgb::RGB8;
use egui::Widget;

use crate::mastermindwidget::{MastermindWidget, GuessState};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    settings: BoardSettings,

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
        let settings = BoardSettings::default()
            .colors(vec![
                RGB8::new(255, 0, 0),
                RGB8::new(255, 0, 255),
                RGB8::new(255, 255, 0),
            ])
            .code_length(4)
            .max_tries(8);
        Self {
            guess_state: GuessState::new(&settings),
            settings: settings.clone(),
            board: MastermindBoard::new(settings),
            current_page: Page::Home,
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
        app.guess_state  = GuessState::new(&app.settings);

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
                MastermindWidget { board: & self.board, guess_state: & mut self.guess_state }.ui(ui);
            }
            Page::Settings => {
                ui.heading("Settings");
                ui.add(
                    egui::Slider::new(&mut self.settings.code_length, 1..=10).text("Code Length"),
                );

                ui.add(
                    egui::Slider::new(&mut self.settings.max_tries, 1..=12).text("Max Tries"),
                );
                ui.separator();
            }
        });
    }
}
