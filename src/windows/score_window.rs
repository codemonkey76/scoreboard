use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow}
};

use egui_multiwin::winit::window::Fullscreen::Borderless;

use crate::app::AppCommon;

pub struct ScoreWindow {
    pub input: String,
    is_fullscreen: bool,
    should_quit: bool
}

impl ScoreWindow {
    pub fn request(label: String) -> NewWindowRequest<AppCommon> {
        NewWindowRequest {
            window_state: Box::new(ScoreWindow {
                input: label.clone(),
                is_fullscreen: false,
                should_quit: false
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(false)
                .with_inner_size(egui_multiwin::winit::dpi::LogicalSize {
                    width: 400.0,
                    height: 200.0
                })
            .with_title(label),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
        }
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
        let mut quit = false;


        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            if ui.input(|i| i.modifiers.alt && i.key_pressed(egui_multiwin::egui::Key::Enter)) {
                self.is_fullscreen = !self.is_fullscreen;

                if self.is_fullscreen {
                    window.set_fullscreen(Some(Borderless(None)));
                } else {
                    window.set_fullscreen(None);
                }
            };
            ui.label(format!("{:?}", c));
        });

         if ! c.show_score_window || self.should_quit {
             quit = true;
         }
         if quit {
             println!("Quitting score window");
         }

        RedrawResponse {
            quit,
            new_windows: Vec::new(),
        }
    }
}
