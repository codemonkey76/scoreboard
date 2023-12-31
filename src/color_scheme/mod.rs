use egui_multiwin::egui::Color32;
use serde::{self, Deserialize, Serialize};

mod serde_color32;

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorScheme {
    pub competitor_1: CompetitorColorScheme,
    pub competitor_2: CompetitorColorScheme,
    #[serde(with = "serde_color32")]
    pub time_bg: Color32,
    #[serde(with = "serde_color32")]
    pub time: Color32,
    #[serde(with = "serde_color32")]
    pub fight_info_heading: Color32,
    #[serde(with = "serde_color32")]
    pub fight_info_sub_heading: Color32,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            competitor_1: Default::default(),
            competitor_2: CompetitorColorScheme {
                bg: Color32::from_rgb(49, 42, 109),
                adv_bg: Color32::from_rgb(49, 42, 109),
                pen_bg: Color32::from_rgb(49, 42, 109),
                points_bg: Color32::from_rgb(46, 100, 209),
                ..Default::default()
            },
            time_bg: Color32::from_rgb(0, 0, 0),
            time: Color32::from_rgb(255, 255, 255),
            fight_info_heading: Color32::from_rgb(200, 200, 140),
            fight_info_sub_heading: Color32::from_rgb(255, 255, 255),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompetitorColorScheme {
    #[serde(with = "serde_color32")]
    pub bg: Color32,
    #[serde(with = "serde_color32")]
    pub name: Color32,
    #[serde(with = "serde_color32")]
    pub team: Color32,
    #[serde(with = "serde_color32")]
    pub adv_bg: Color32,
    #[serde(with = "serde_color32")]
    pub adv: Color32,
    #[serde(with = "serde_color32")]
    pub pen_bg: Color32,
    #[serde(with = "serde_color32")]
    pub pen: Color32,
    #[serde(with = "serde_color32")]
    pub points_bg: Color32,
    #[serde(with = "serde_color32")]
    pub points: Color32,
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
