use egui_multiwin::egui::{Color32, Rounding, Stroke};
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};

use egui_multiwin::winit::window::Fullscreen::Borderless;

use crate::app::AppCommon;
use crate::score_grid::ScoreGrid;

pub struct ScoreWindow {
    pub window_title: String,
    is_fullscreen: bool,
}

impl ScoreWindow {
    pub fn request(label: String) -> NewWindowRequest<AppCommon> {
        NewWindowRequest {
            window_state: Box::new(ScoreWindow {
                window_title: label.clone(),
                is_fullscreen: false,
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

    fn main_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            c.score_grids = Some(ScoreGrid::calc_grids(ui.clip_rect(), &c.grid_config));

            if let Some(grids) = &c.score_grids {
                ui.painter().rect_stroke(
                    grids.top,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );
                ui.painter().rect_stroke(
                    grids.middle,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );
                ui.painter().rect_stroke(
                    grids.bottom,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_one_name,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_one_flag,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_one_team,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_one_score,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_one_advantage,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_one_penalty,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_two_name,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_two_flag,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_two_team,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_two_score,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_two_advantage,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.competitor_two_penalty,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.timer,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.fight_info,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );

                ui.painter().rect_stroke(
                    grids.fight_sub_info,
                    Rounding::none(),
                    Stroke::new(1.0, Color32::WHITE),
                );
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

        RedrawResponse {
            quit: !c.show_score_window,
            new_windows: vec![],
        }
    }
}
