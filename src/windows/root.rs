use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};

use crate::app::AppCommon;
use super::popup_window::PopupWindow;

pub struct RootWindow {
    pub button_pressed_count: u32,
    pub num_popups_created: u32
}

impl RootWindow {
    pub fn request() -> NewWindowRequest<AppCommon> {
        NewWindowRequest {
            window_state: Box::new(RootWindow {
                button_pressed_count: 0,
                num_popups_created: 0,
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

        egui_multiwin::egui::SidePanel::left("my_side_panel").show(&egui.egui_ctx, |ui| {
            ui.heading("Hello World!");
            if ui.button("New popup").clicked() {
                windows_to_create.push(PopupWindow::request(format!(
                    "popup window #{}",
                    self.num_popups_created
                )));
                self.num_popups_created += 1;
            }
            if ui.button("Quit").clicked() {
                quit = true;
            }
        });

        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            if ui.button("Quit").clicked() {
                quit = true;
            }
            ui.heading(format!("number {}", c.clicks));
        });
        RedrawResponse {
            quit,
            new_windows: windows_to_create,
        }
    }
}
