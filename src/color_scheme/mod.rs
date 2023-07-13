use egui_multiwin::egui::Color32;
use serde::{self, Deserialize, Serialize};

mod serde_color32;

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
    #[serde(with = "serde_color32")]
    bg: Color32,
    #[serde(with = "serde_color32")]
    name: Color32,
    #[serde(with = "serde_color32")]
    team: Color32,
    #[serde(with = "serde_color32")]
    adv_bg: Color32,
    #[serde(with = "serde_color32")]
    adv: Color32,
    #[serde(with = "serde_color32")]
    pen_bg: Color32,
    #[serde(with = "serde_color32")]
    pen: Color32,
    #[serde(with = "serde_color32")]
    points_bg: Color32,
    #[serde(with = "serde_color32")]
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
