use egui_multiwin::egui::{Color32, Context, pos2, Rect, TextureHandle, TextureId, TextureOptions, Ui};


pub struct ImageWidget {
    pub name: String,
    pub image_data: &'static [u8],
    pub rect: Rect,
    texture_handle: Option<TextureHandle>,
}

impl ImageWidget {
    pub fn new(name: String, image_data: &'static [u8], rect: Rect) -> Self {
        Self {
            name,
            image_data,
            rect,
            texture_handle: None,
        }
    }

    pub fn load_texture(&mut self, ctx: &Context) {
        if self.texture_handle.is_none() {
            if let Ok(image) = egui_extras::image::load_svg_bytes_with_size(
                self.image_data,
                egui_extras::image::FitTo::Height(360)
            ) {
                self.texture_handle = Some(ctx.load_texture(self.name.clone(), image, TextureOptions::default()));
            }
        }
    }

    pub fn draw(&self, ui: &mut Ui, _scale: f32) {
        if let Some(handle) = &self.texture_handle {
            ui.painter().image(handle.id(), self.rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
    }
}

