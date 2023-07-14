use egui_multiwin::multi_window::MultiWindow;

pub mod app;
pub mod windows;
pub mod color_scheme;
pub mod bjj_match;

use app::AppCommon;

use windows::root_window;
use crate::app::WindowEvent;


const SCORE_FONT: &[u8] = include_bytes!("../assets/fonts/BebasNeue-Regular.ttf");

fn main() {
   let mut event_loop = egui_multiwin::winit::event_loop::EventLoopBuilder::with_user_event();

   #[cfg(target_os = "linux")]
   egui_multiwin::winit::platform::x11::EventLoopBuilderExtX11::with_x11(&mut event_loop);

   let event_loop = event_loop.build();


  let mut multi_window: MultiWindow<AppCommon, WindowEvent> = MultiWindow::new();
   multi_window.add_font("score_font".to_string(), egui_multiwin::egui::FontData::from_static(SCORE_FONT));
   let root_window = root_window::RootWindow::request();


    let ac = AppCommon::from_project_dirs();

    //let toml = toml::to_string(&ac.color_scheme).unwrap();
    // let ac = toml::from_str(toml.as_str());
    // println!("{:?}", ac);

   let _e = multi_window.add(root_window, &event_loop);

   multi_window.run(event_loop, ac);
    
}
