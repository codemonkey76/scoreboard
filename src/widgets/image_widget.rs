use egui_multiwin::egui::{Color32, pos2, Rect, TextureHandle, Ui};


pub struct ImageWidget {
    pub name: String,
    pub rect: Rect,
    texture_handle: Option<TextureHandle>,
}

impl ImageWidget {
    pub fn new(name: String, texture_handle: Option<TextureHandle>, rect: Rect) -> Self {
        Self {
            name,
            rect,
            texture_handle,
        }
    }

    pub fn draw(&self, ui: &mut Ui, _scale: f32) {
        if let Some(handle) = &self.texture_handle {
            ui.painter().image(handle.id(), self.rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
    }
}

