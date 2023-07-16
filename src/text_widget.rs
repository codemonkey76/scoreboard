use crate::score_grid::CalculatePosition;
use egui_multiwin::egui::{Align2, Color32, FontId, Rect, Ui};

pub struct TextWidget {
    pub text: String,
    pub alignment: Align2,
    pub rect: Rect,
    pub font_size: f32,
    pub font: FontId,
    pub padding: f32,
    pub color: Color32,
}

impl TextWidget {
    pub fn draw(&self, ui: &mut Ui, scale: f32) {
        let pos = self.rect.calc_pos(self.alignment, self.padding * scale);

        let font = FontId {
            size: self.font_size * scale,
            ..self.font.clone()
        };

        ui.painter()
            .text(pos, self.alignment, &self.text, font, self.color);
    }
}
