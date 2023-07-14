use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};
use egui_extras::{Column, StripBuilder, TableBuilder};

use crate::app::AppCommon;
use super::score_window::ScoreWindow;

pub struct RootWindow {
    show_matches: bool,
}

impl RootWindow {
    pub fn request() -> NewWindowRequest<AppCommon> {
        NewWindowRequest {
            window_state: Box::new(RootWindow {
                show_matches: true
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(egui_multiwin::winit::dpi::LogicalSize {
                    width: 800.0,
                    height: 400.0
                })
            .with_title("BJJ Scoreboard - Controls"),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            }
        }
    }
    pub fn table_ui(&mut self, ui: &mut egui_multiwin::egui::Ui) {

    }
}
impl TrackedWindow<AppCommon> for RootWindow {
    fn is_root(&self) -> bool {
        true
    }

    fn set_root(&mut self, _root: bool) {}

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        _window: &egui_multiwin::winit::window::Window,
    ) -> RedrawResponse<AppCommon> {
        let mut quit = false;

        let mut windows_to_create = vec![];

        egui_multiwin::egui::TopBottomPanel::top("Top Bar").show(&egui.egui_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(quit, "Quit").clicked() {
                    quit = true;
                }
                if ui.selectable_label(self.show_matches, "🎮 Matches").clicked() {
                    self.show_matches = !self.show_matches
                }
                if ui.selectable_label(c.show_score_window, "🕑 Show Scores Window").clicked() {
                    c.show_score_window = ! c.show_score_window;

                    if c.show_score_window {
                        windows_to_create.push(ScoreWindow::request("Score Window".to_string()));
                    }
                }
            });
        });


        egui_multiwin::egui::Window::new("🎮 Matches")
            .open(&mut self.show_matches)
            .vscroll(true)
            .show(&egui.egui_ctx, |ui| {
                StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            egui_multiwin::egui::ScrollArea::horizontal().show(ui, |ui| {
                                let mut table = TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(egui_multiwin::egui::Layout::left_to_right(egui_multiwin::egui::Align::Center))
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .min_scrolled_height(0.0);
                                table.header(20.0, |mut header| {
                                   header.col(|ui| {
                                       ui.strong("Row");
                                   });
                                })
                                    .body(|mut body| {
                                        for match_row in &c.matches {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| {
                                                    let label_string = format!("{:?}", match_row).to_string();
                                                    ui.label(label_string);
                                                });
                                            });

                                        }
                                    })
                            });
                        })
                    });
            });

        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            ui.label(format!("{:?}", c.color_scheme));
            egui_multiwin::egui::Frame::default().show(ui, |ui| {
                ui.label("Testing");
            });
        });

        RedrawResponse {
            quit,
            new_windows: windows_to_create,
        }
    }
}
