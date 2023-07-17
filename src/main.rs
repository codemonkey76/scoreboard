use egui_multiwin::multi_window::MultiWindow;

pub mod app;
pub mod bjj_match;
pub mod color_scheme;
pub mod font_config;
pub mod score_grid;
pub mod windows;
pub mod countries;
pub mod flag;
pub mod widgets;

use app::AppCommon;

use crate::app::WindowEvent;
use windows::root_window;

const SCORE_FONT: &[u8] = include_bytes!("../assets/fonts/BebasNeue-Regular.ttf");

fn main() {
    let mut event_loop = egui_multiwin::winit::event_loop::EventLoopBuilder::with_user_event();

    #[cfg(target_os = "linux")]
    egui_multiwin::winit::platform::x11::EventLoopBuilderExtX11::with_x11(&mut event_loop);

    let event_loop = event_loop.build();

    let mut multi_window: MultiWindow<AppCommon, WindowEvent> = MultiWindow::new();

    let mut font_data = egui_multiwin::egui::FontData::from_static(SCORE_FONT);
    font_data.tweak.y_offset_factor = 0.08;

    multi_window.add_font("score_font".to_string(), font_data);

    let root_window = root_window::RootWindow::request();
    let ac = AppCommon::from_project_dirs();

    let _e = multi_window.add(root_window, &event_loop);

    multi_window.run(event_loop, ac);
}
