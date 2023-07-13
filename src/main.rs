use egui_multiwin::multi_window::MultiWindow;

pub mod app;
pub mod windows;
pub mod color_scheme;

use app::AppCommon;

use windows::{root, popup_window};

const COMPUTER_MODERN_FONT: &[u8] = include_bytes!("../assets/fonts/BebasNeue-Regular.ttf");

fn main() {
//    let mut event_loop = egui_multiwin::winit::event_loop::EventLoopBuilder::with_user_event();

//    #[cfg(target_os = "linux")]
//    egui_multiwin::winit::platform::x11::EventLoopBuilderExtX11::with_x11(&mut event_loop);

//    let event_loop = event_loop.build();
//    let proxy = event_loop.create_proxy();

//    if let Err(e) = proxy.send_event(42) {
//        println!("Failed to send event loop message: {:?}", e);
//    }

//   let mut multi_window: MultiWindow<AppCommon, u32> = MultiWindow::new();
//    multi_window.add_font("computermodern".to_string(), egui_multiwin::egui::FontData::from_static(COMPUTER_MODERN_FONT));
//    let root_window = root::RootWindow::request();
//    let root_window2 = popup_window::PopupWindow::request("BJJ Scoreboard - Scores".to_string());

    let ac = AppCommon { color_scheme: Default::default() };
    let toml = toml::to_string(&ac).unwrap();
    println!("{}", toml);

//    let _e = multi_window.add(root_window, &event_loop);
  //  let _e = multi_window.add(root_window2, &event_loop);
   // multi_window.run(event_loop, ac);
    
}
