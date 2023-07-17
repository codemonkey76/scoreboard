use crate::score_grid::CalculatePosition;
use egui_multiwin::egui::{Align2, Color32, FontId, Rect, Ui};

pub struct TextWidget {
    pub text: String,
    pub alignment: Align2,
    pub rect: Rect,
    pub font_size: f32,
    pub font: FontId,
    pub padding: Padding,
    pub color: Color32,
}

impl TextWidget {
    pub fn draw(&self, ui: &mut Ui, scale: f32) {
        let pos = self.rect.calc_pos(self.alignment, self.padding, scale);

        let font = FontId {
            size: self.font_size * scale,
            ..self.font.clone()
        };

        ui.painter()
            .text(pos, self.alignment, &self.text, font, self.color);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Padding {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32
}

impl Padding {
    pub fn x(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: amount,
            right: amount
        }
    }
    pub fn y(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: amount,
            left: 0.0,
            right: 0.0
        }
    }
    pub fn all(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: amount,
            left: amount,
            right: amount
        }
    }
    pub fn none() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0
        }
    }

    pub fn left(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: amount,
            right: 0.0
        }
    }

    pub fn right(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: amount
        }
    }

    pub fn top(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: 0.0,
            left: 0.0,
            right: 0.0
        }
    }

    pub fn bottom(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: amount,
            left: 0.0,
            right: 0.0
        }
    }
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self{
            top, bottom, left, right
        }
    }
}
