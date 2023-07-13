use egui_multiwin::egui::Color32;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct ColorScheme {
    competitor_1: CompetitorColorScheme,
    competitor_2: CompetitorColorScheme
}


impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            competitor_1: Default::default(),
            competitor_2: Default::default()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompetitorColorScheme {
    bg: Color32,
    name: Color32,
    team: Color32,
    adv_bg: Color32,
    adv: Color32,
    pen_bg: Color32,
    pen: Color32,
    points_bg: Color32,
    points: Color32
}

impl Default for CompetitorColorScheme {
    fn default() -> Self {
        Self {
            bg: Color32::from_rgb(0, 0, 0),
            name: Color32::from_rgb(255, 255, 255),
            team: Color32::from_rgb(255, 255, 255),
            adv_bg: Color32::from_rgb(0, 0, 0),
            adv: Color32::from_rgb(255, 255, 255),
            pen_bg: Color32::from_rgb(0, 0, 0),
            pen: Color32::from_rgb(255, 255, 255),
            points_bg: Color32::from_rgb(227, 85, 141),
            points: Color32::from_rgb(255, 255, 255),
        }
    }
}
