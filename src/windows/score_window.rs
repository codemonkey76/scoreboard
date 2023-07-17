use egui_multiwin::egui::{Align2, Color32, FontId, Rounding, Stroke, Ui};
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};

use egui_multiwin::winit::window::Fullscreen::Borderless;

use crate::app::AppCommon;
use crate::bjj_match::{BjjMatch, MatchState};
use crate::score_grid::ScoreGrid;
use crate::text_widget::{Padding, TextWidget};

pub struct ScoreWindow {
    is_fullscreen: bool,
    scale: f32,
    score_grid: ScoreGrid,
    widgets: Vec<TextWidget>,
}

impl ScoreWindow {
    pub fn request(label: String) -> NewWindowRequest<AppCommon> {
        NewWindowRequest {
            window_state: Box::new(ScoreWindow {
                is_fullscreen: false,
                scale: 1.0,
                score_grid: Default::default(),
                widgets: vec![],
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(egui_multiwin::winit::dpi::LogicalSize {
                    width: 720.0,
                    height: 405.0,
                })
                .with_title(label),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
        }
    }

    fn sync_fullscreen_mode(&mut self, window: &egui_multiwin::winit::window::Window) {
        if self.is_fullscreen {
            window.set_fullscreen(Some(Borderless(None)));
        } else {
            window.set_fullscreen(None);
        }
    }

    fn handle_input(&mut self, _: &mut AppCommon, egui: &mut EguiGlow) {
        if egui
            .egui_ctx
            .input(|i| i.modifiers.alt && i.key_pressed(egui_multiwin::egui::Key::Enter))
        {
            self.is_fullscreen = !self.is_fullscreen;
        };
    }

    fn draw_grid(&mut self, ui: &mut Ui, c: &mut AppCommon) {
        let grid = &self.score_grid;

        ui.painter()
            .rect_filled(grid.top, Rounding::none(), c.color_scheme.competitor_1.bg);

        ui.painter().rect_filled(
            grid.middle,
            Rounding::none(),
            c.color_scheme.competitor_2.bg,
        );

        ui.painter()
            .rect_filled(grid.bottom, Rounding::none(), c.color_scheme.time_bg);

        ui.painter().rect_filled(
            grid.competitor_one_score,
            Rounding::none(),
            c.color_scheme.competitor_1.points_bg,
        );

        ui.painter().rect_filled(
            grid.competitor_two_score,
            Rounding::none(),
            c.color_scheme.competitor_2.points_bg,
        );
    }

    fn draw_debug_grid(&mut self, ui: &mut Ui) {
        let grid = &self.score_grid;

        ui.painter().rect_stroke(
            grid.competitor_one_name,
            Rounding::none(),
            Stroke::new(2.0, Color32::GREEN),
        );
        ui.painter().rect_stroke(
            grid.competitor_one_team,
            Rounding::none(),
            Stroke::new(2.0, Color32::GREEN),
        );

        ui.painter().rect_stroke(
            grid.competitor_one_flag,
            Rounding::none(),
            Stroke::new(2.0, Color32::GREEN),
        );

        ui.painter().rect_stroke(
            grid.competitor_one_advantage,
            Rounding::none(),
            Stroke::new(2.0, Color32::GREEN),
        );

        ui.painter().rect_stroke(
            grid.competitor_one_penalty,
            Rounding::none(),
            Stroke::new(2.0, Color32::GREEN),
        );

        ui.painter().rect_stroke(
            grid.competitor_one_score,
            Rounding::none(),
            Stroke::new(2.0, Color32::GREEN),
        );

        ui.painter().rect_stroke(
            grid.competitor_two_name,
            Rounding::none(),
            Stroke::new(2.0, Color32::RED),
        );
        ui.painter().rect_stroke(
            grid.competitor_two_team,
            Rounding::none(),
            Stroke::new(2.0, Color32::RED),
        );

        ui.painter().rect_stroke(
            grid.competitor_two_flag,
            Rounding::none(),
            Stroke::new(2.0, Color32::RED),
        );

        ui.painter().rect_stroke(
            grid.competitor_two_advantage,
            Rounding::none(),
            Stroke::new(2.0, Color32::RED),
        );

        ui.painter().rect_stroke(
            grid.competitor_two_penalty,
            Rounding::none(),
            Stroke::new(2.0, Color32::RED),
        );

        ui.painter().rect_stroke(
            grid.competitor_two_score,
            Rounding::none(),
            Stroke::new(2.0, Color32::RED),
        );

        ui.painter().rect_stroke(
            grid.timer,
            Rounding::none(),
            Stroke::new(2.0, Color32::BLUE),
        );

        ui.painter().rect_stroke(
            grid.fight_info,
            Rounding::none(),
            Stroke::new(2.0, Color32::BLUE),
        );

        ui.painter().rect_stroke(
            grid.fight_sub_info,
            Rounding::none(),
            Stroke::new(2.0, Color32::BLUE),
        );

        ui.painter()
            .rect_stroke(grid.logo, Rounding::none(), Stroke::new(2.0, Color32::BLUE));
    }

    fn calculate_score_grid(&mut self, c: &mut AppCommon, ui: &mut Ui) -> ScoreGrid {
        // Re-run this anytime the grid config changes, or the screen is resized
        ScoreGrid::calc_grids(ui.clip_rect(), &c.grid_config)
    }

    fn create_widgets(&mut self, c: &AppCommon, bjj_match: &BjjMatch) {
        self.widgets.clear();
        // Create the widgets everytime the match changes
        // Recalculate teh rectangles if any of the grid changes, or if the screen is resized.
        // As naive implementation we will just do this every frame.
        let font = FontId {
            size: 12.0,
            family: egui_multiwin::egui::FontFamily::Name("score_font".into()),
        };

        self.widgets.push(TextWidget {
            text: bjj_match.competitor_one.display_name(),
            alignment: Align2::LEFT_CENTER,
            rect: self.score_grid.competitor_one_name,
            font_size: c.font_config.competitor_name,
            font: font.clone(),
            padding: Padding::left(4.0),
            color: c.color_scheme.competitor_1.name,
        });

        self.widgets.push(TextWidget {
            text: bjj_match.competitor_two.display_name(),
            alignment: Align2::LEFT_CENTER,
            rect: self.score_grid.competitor_two_name,
            font_size: c.font_config.competitor_name,
            font: font.clone(),
            padding: Padding::left(4.0),
            color: c.color_scheme.competitor_2.name,
        });

        self.widgets.push(TextWidget {
            text: bjj_match.competitor_one.team.to_owned(),
            alignment: Align2::LEFT_CENTER,
            rect: self.score_grid.competitor_one_team,
            font_size: c.font_config.team_name,
            font: font.clone(),
            padding: Padding::left(4.0),
            color: c.color_scheme.competitor_1.team,
        });

        self.widgets.push(TextWidget {
            text: bjj_match.competitor_two.team.to_owned(),
            alignment: Align2::LEFT_CENTER,
            rect: self.score_grid.competitor_two_team,
            font_size: c.font_config.team_name,
            font: font.clone(),
            padding: Padding::left(4.0),
            color: c.color_scheme.competitor_2.team,
        });

        self.widgets.push(TextWidget {
            text: "Adv.".to_owned(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_one_advantage,
            font_size: c.font_config.advantage_label,
            font: font.clone(),
            padding: Padding::top(1.5),
            color: c.color_scheme.competitor_1.adv,
        });

        self.widgets.push(TextWidget {
            text: "Pen.".to_owned(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_one_penalty,
            font_size: c.font_config.penalty_label,
            font: font.clone(),
            padding: Padding::top(1.5),
            color: c.color_scheme.competitor_1.pen,
        });

        self.widgets.push(TextWidget {
            text: "Adv.".to_owned(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_two_advantage,
            font_size: c.font_config.advantage_label,
            font: font.clone(),
            padding: Padding::top(1.5),
            color: c.color_scheme.competitor_2.adv,
        });

        self.widgets.push(TextWidget {
            text: "Pen.".to_owned(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_two_penalty,
            font_size: c.font_config.penalty_label,
            font: font.clone(),
            padding: Padding::top(1.5),
            color: c.color_scheme.competitor_2.pen,
        });

        self.widgets.push(TextWidget {
            text: bjj_match.fight_info.to_owned(),
            alignment: Align2::LEFT_BOTTOM,
            rect: self.score_grid.fight_info,
            font_size: c.font_config.fight_info,
            font: font.clone(),
            padding: Padding::new(4.0, 0.0, 0.0, -2.0),
            color: c.color_scheme.fight_info_heading,
        });

        self.widgets.push(TextWidget {
            text: bjj_match.fight_sub_info.to_owned(),
            alignment: Align2::LEFT_TOP,
            rect: self.score_grid.fight_sub_info,
            font_size: c.font_config.fight_sub_info,
            font: font.clone(),
            padding: Padding::left(4.0),
            color: c.color_scheme.fight_info_sub_heading,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.get_formatted_remaining_time(),
            alignment: Align2::CENTER_CENTER,
            rect: self.score_grid.timer,
            font_size: c.font_config.time,
            font: font.clone(),
            padding: Padding::none(),
            color: c.color_scheme.time,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.competitor_one.advantages.to_string(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_one_advantage,
            font_size: c.font_config.advantage,
            font: font.clone(),
            padding: Padding::top(10.0),
            color: c.color_scheme.competitor_1.adv,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.competitor_one.penalties.to_string(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_one_penalty,
            font_size: c.font_config.penalty,
            font: font.clone(),
            padding: Padding::top(10.0),
            color: c.color_scheme.competitor_2.pen,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.competitor_two.advantages.to_string(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_two_advantage,
            font_size: c.font_config.advantage,
            font: font.clone(),
            padding: Padding::top(10.0),
            color: c.color_scheme.competitor_2.adv,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.competitor_two.penalties.to_string(),
            alignment: Align2::CENTER_TOP,
            rect: self.score_grid.competitor_two_penalty,
            font_size: c.font_config.penalty,
            font: font.clone(),
            padding: Padding::top(10.0),
            color: c.color_scheme.competitor_2.pen,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.competitor_one.points.to_string(),
            alignment: Align2::CENTER_CENTER,
            rect: self.score_grid.competitor_one_score,
            font_size: c.font_config.points,
            font: font.clone(),
            padding: Padding::none(),
            color: c.color_scheme.competitor_1.points,
        });

        self.widgets.push(TextWidget {
            text: c.match_info.competitor_two.points.to_string(),
            alignment: Align2::CENTER_CENTER,
            rect: self.score_grid.competitor_two_score,
            font_size: c.font_config.points,
            font: font.clone(),
            padding: Padding::none(),
            color: c.color_scheme.competitor_2.points,
        });
    }

    fn draw_widgets(&mut self, ui: &mut Ui, scale: f32) {
        for widget in &self.widgets {
            widget.draw(ui, scale);
        }
    }

    fn main_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            self.scale = ui.clip_rect().width() / 400.0 * c.font_config.scale;
            self.score_grid = self.calculate_score_grid(c, ui);
            self.draw_grid(ui, c);
            if c.show_debug_grid {
                self.draw_debug_grid(ui);
            }

            if let Some(bjj_match) = &c.selected_match {
                self.create_widgets(c, bjj_match);
                self.draw_widgets(ui, self.scale);
            }
        });
    }
}

impl TrackedWindow<AppCommon> for ScoreWindow {
    fn can_quit(&self, c: &mut AppCommon) -> bool {
        c.show_score_window = false;

        true
    }

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        window: &egui_multiwin::winit::window::Window,
    ) -> RedrawResponse<AppCommon> {
        self.sync_fullscreen_mode(window);
        self.main_ui(c, egui);
        self.handle_input(c, egui);
        if c.match_info.match_state == MatchState::InProgress {
            egui.egui_ctx.request_repaint();
        }

        RedrawResponse {
            quit: !c.show_score_window,
            new_windows: vec![],
        }
    }
}
