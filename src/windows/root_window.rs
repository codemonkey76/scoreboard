use egui_extras::{Column, StripBuilder, TableBuilder};
use egui_multiwin::egui::DragValue;
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};

use super::score_window::ScoreWindow;
use crate::app::AppCommon;
use crate::bjj_match::PointType;

pub struct RootWindow {
    show_matches: bool,
    show_grid_config: bool,
    show_font_config: bool,
    show_match_controls: bool,
}

impl RootWindow {
    pub fn request() -> NewWindowRequest<AppCommon> {
        NewWindowRequest {
            window_state: Box::new(RootWindow {
                show_matches: false,
                show_grid_config: true,
                show_font_config: true,
                show_match_controls: false,
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(egui_multiwin::winit::dpi::LogicalSize {
                    width: 1280.0,
                    height: 720.0,
                })
                .with_title("BJJ Scoreboard - Controls"),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
        }
    }

    pub fn matches_table_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::Window::new("Matches")
            .open(&mut self.show_matches)
            .vscroll(true)
            .min_width(700.0)
            .show(&egui.egui_ctx, |ui| {
                StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            egui_multiwin::egui::ScrollArea::horizontal().show(ui, |ui| {
                                let table = TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(egui_multiwin::egui::Layout::left_to_right(
                                        egui_multiwin::egui::Align::Center,
                                    ))
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .column(Column::auto())
                                    .min_scrolled_height(0.0);
                                table
                                    .header(20.0, |mut header| {
                                        header.col(|ui| {
                                            ui.strong("Select Match");
                                        });
                                        header.col(|ui| {
                                            ui.strong("Competitor 1");
                                        });
                                        header.col(|ui| {
                                            ui.strong("Competitor 2");
                                        });
                                        header.col(|ui| {
                                            ui.strong("Match Duration");
                                        });
                                        header.col(|ui| {
                                            ui.strong("Fight Info");
                                        });
                                        header.col(|ui| {
                                            ui.strong("Fight Sub Info");
                                        });
                                    })
                                    .body(|mut body| {
                                        for match_row in &c.matches {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| {
                                                    if ui.button("Select").clicked() {
                                                        c.selected_match = Some(match_row.clone());
                                                        c.new_match = true;
                                                    }
                                                });
                                                row.col(|ui| {
                                                    ui.label(format!(
                                                        "{}",
                                                        match_row.competitor_one
                                                    ));
                                                });

                                                row.col(|ui| {
                                                    ui.label(format!(
                                                        "{}",
                                                        match_row.competitor_two
                                                    ));
                                                });

                                                row.col(|ui| {
                                                    ui.label(format!("{}", match_row.time_limit));
                                                });

                                                row.col(|ui| {
                                                    ui.label(match_row.fight_info.to_string());
                                                });

                                                row.col(|ui| {
                                                    ui.label(match_row.fight_sub_info.to_string());
                                                });
                                            });
                                        }
                                    })
                            });
                        })
                    });
            });
    }

    pub fn match_controls_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::Window::new("Match Controls")
            .open(&mut self.show_match_controls)
            .vscroll(true)
            .min_width(700.0)
            .show(&egui.egui_ctx, |ui| {
                egui_multiwin::egui::Grid::new("Match Controls")
                    .num_columns(9)
                    .spacing([25.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Competitor 1");
                        if ui.button("+2").clicked() {
                            c.match_info.competitor_one.add(PointType::Point, 2);
                        }
                        if ui.button("+3").clicked() {
                            c.match_info.competitor_one.add(PointType::Point, 3);
                        }
                        if ui.button("+4").clicked() {
                            c.match_info.competitor_one.add(PointType::Point, 4);
                        }
                        if ui.button("-1").clicked() {
                            c.match_info.competitor_one.subtract(PointType::Point);
                        }
                        if ui.button("A").clicked() {
                            c.match_info.competitor_one.add(PointType::Advantage, 1);
                        }
                        if ui.button("-A").clicked() {
                            c.match_info.competitor_one.subtract(PointType::Advantage);
                        }
                        if ui.button("P").clicked() {
                            c.match_info.competitor_one.add(PointType::Penalty, 1);
                        }
                        if ui.button("-P").clicked() {
                            c.match_info.competitor_one.subtract(PointType::Penalty);
                        }
                        ui.end_row();

                        ui.label("Competitor 2");
                        if ui.button("+2").clicked() {
                            c.match_info.competitor_two.add(PointType::Point, 2);
                        }
                        if ui.button("+3").clicked() {
                            c.match_info.competitor_two.add(PointType::Point, 3);
                        }
                        if ui.button("+4").clicked() {
                            c.match_info.competitor_two.add(PointType::Point, 4);
                        }
                        if ui.button("-1").clicked() {
                            c.match_info.competitor_two.subtract(PointType::Point);
                        }
                        if ui.button("A").clicked() {
                            c.match_info.competitor_two.add(PointType::Advantage, 1);
                        }
                        if ui.button("-A").clicked() {
                            c.match_info.competitor_two.subtract(PointType::Advantage);
                        }
                        if ui.button("P").clicked() {
                            c.match_info.competitor_two.add(PointType::Penalty, 1);
                        }
                        if ui.button("-P").clicked() {
                            c.match_info.competitor_two.subtract(PointType::Penalty);
                        }
                        ui.end_row();

                        ui.label("Match Controls");
                        if ui.button("Start Match").clicked() {
                            c.match_info.start();
                        }
                        if ui.button("End Match").clicked() {
                            c.match_info.stop();
                        }
                        if ui.button("Pause Match").clicked() {
                            c.match_info.pause();
                        }

                    });
            });
    }

    pub fn grid_config_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::Window::new("Grid Config")
            .open(&mut self.show_grid_config)
            .vscroll(true)
            .min_width(700.0)
            .show(&egui.egui_ctx, |ui| {
                egui_multiwin::egui::Grid::new("Grid Settings")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Time Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.competitor_time)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Points Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.points)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Advantage/Penalty Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.name_adv)
                                .speed(0.01)
                                .clamp_range(0.05..=0.50),
                        );
                        ui.end_row();

                        ui.label("Flag Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.flag_name)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Name / Team Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.name_team)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Timer Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.timer_fight_info)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Fight Info / Sub Info Proportion");
                        ui.add(
                            DragValue::new(&mut c.grid_config.fight_info_sub_info)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();
                    });
            });
    }

    pub fn font_config_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::Window::new("Font Config")
            .open(&mut self.show_font_config)
            .vscroll(true)
            .min_width(700.0)
            .show(&egui.egui_ctx, |ui| {
                egui_multiwin::egui::Grid::new("Font Settings")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Global scaling");
                        ui.add(
                            DragValue::new(&mut c.font_config.scale)
                                .speed(0.01)
                                .clamp_range(0.1..=5.0),
                        );
                        ui.end_row();

                        ui.label("Competitor Name");
                        ui.add(
                            DragValue::new(&mut c.font_config.competitor_name)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Team Name");
                        ui.add(
                            DragValue::new(&mut c.font_config.team_name)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Points");
                        ui.add(
                            DragValue::new(&mut c.font_config.points)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Team");
                        ui.add(
                            DragValue::new(&mut c.font_config.team_name)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Adv. Label");
                        ui.add(
                            DragValue::new(&mut c.font_config.advantage_label)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Advantage");
                        ui.add(
                            DragValue::new(&mut c.font_config.advantage)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Pen. Label");
                        ui.add(
                            DragValue::new(&mut c.font_config.penalty_label)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Penalty");
                        ui.add(
                            DragValue::new(&mut c.font_config.penalty)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Timer");
                        ui.add(
                            DragValue::new(&mut c.font_config.time)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Fight Info");
                        ui.add(
                            DragValue::new(&mut c.font_config.fight_info)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();

                        ui.label("Fight Sub Info");
                        ui.add(
                            DragValue::new(&mut c.font_config.fight_sub_info)
                                .speed(0.1)
                                .clamp_range(1.0..=72.0),
                        );
                        ui.end_row();
                    });
            });
    }

    pub fn central_panel_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            if let Some(bjj_match) = &c.selected_match {
                ui.label(format!("Selected Match: {}", bjj_match.competitor_one));
            }
        });
    }

    pub fn menu_bar_ui(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
    ) -> RedrawResponse<AppCommon> {
        let mut quit = false;

        let mut windows_to_create = vec![];

        egui_multiwin::egui::TopBottomPanel::top("Top Bar").show(&egui.egui_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(quit, "Quit").clicked() {
                    quit = true;
                }

                ui.checkbox(&mut c.show_debug_grid, "Debug Grid");

                if ui
                    .selectable_label(self.show_matches, "🎮 Matches")
                    .clicked()
                {
                    self.show_matches = !self.show_matches
                }
                if ui
                    .selectable_label(c.show_score_window, "🕑 Show Scores Window")
                    .clicked()
                {
                    c.show_score_window = !c.show_score_window;

                    if c.show_score_window {
                        windows_to_create.push(ScoreWindow::request("Score Window".to_string()));
                    }
                }

                if ui
                    .selectable_label(self.show_grid_config, "▦ Grid Config")
                    .clicked()
                {
                    self.show_grid_config = !self.show_grid_config
                }
                if ui
                    .selectable_label(self.show_grid_config, "ⓕ Font Config")
                    .clicked()
                {
                    self.show_font_config = !self.show_font_config
                }
                if ui
                    .selectable_label(self.show_match_controls, "🎮 Match Controls")
                    .clicked()
                {
                    self.show_match_controls = !self.show_match_controls
                }
            });
        });

        RedrawResponse {
            quit,
            new_windows: windows_to_create,
        }
    }
}
impl TrackedWindow<AppCommon> for RootWindow {
    fn is_root(&self) -> bool {
        true
    }

    fn can_quit(&self, c: &mut AppCommon) -> bool {
        c.save();

        true
    }

    fn set_root(&mut self, _root: bool) {}

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        _window: &egui_multiwin::winit::window::Window,
    ) -> RedrawResponse<AppCommon> {
        let response = self.menu_bar_ui(c, egui);
        self.matches_table_ui(c, egui);
        self.match_controls_ui(c, egui);
        self.grid_config_ui(c, egui);
        self.font_config_ui(c, egui);
        self.central_panel_ui(c, egui);

        // Returns the RedrawResponse
        response
    }
}
