use egui_multiwin::egui::Ui;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::text_widget::TextWidget;

pub mod text_widget;
pub mod image_widget;

pub enum Widget {
    Text(TextWidget),
    Image(ImageWidget)
}

impl Widget {
    pub fn draw(&mut self, ui: &mut Ui, scale: f32) {
        match self {
            Widget::Text(tw) => tw.draw(ui, scale),
            Widget::Image(iw) => iw.draw(ui, scale)
        }
    }
}