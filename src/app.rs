use crate::windows::popup_window;

pub struct AppCommon {
    pub clicks: u32,
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
