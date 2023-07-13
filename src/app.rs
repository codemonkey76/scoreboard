use crate::{windows::popup_window, color_scheme::ColorScheme};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppCommon {
    //All application information is stored here.
    pub color_scheme: ColorScheme
}

impl egui_multiwin::multi_window::CommonEventHandler<AppCommon, u32> for AppCommon {
    fn process_event(&mut self, event: u32) -> Vec<egui_multiwin::multi_window::NewWindowRequest<AppCommon>> {
        let mut windows_to_create = vec![];
        println!("Received an event {}", event);
        match event {
            42 => windows_to_create.push(popup_window::PopupWindow::request("event popup".to_string())),
            _ => {}
        }
        windows_to_create
    }
}
