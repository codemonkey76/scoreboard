use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FontConfig {
    pub scale: f32,
    pub competitor_name: f32,
    pub team_name: f32,
    pub points: f32,
    pub advantage: f32,
    pub advantage_label: f32,
    pub penalty: f32,
    pub penalty_label: f32,
    pub time: f32,
    pub fight_info: f32,
    pub fight_sub_info: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            scale: 1.0,
            competitor_name: 32.0,
            team_name: 28.0,
            points: 120.0,
            advantage: 24.0,
            advantage_label: 8.0,
            penalty: 24.0,
            penalty_label: 8.0,
            time: 28.0,
            fight_info: 28.0,
            fight_sub_info: 24.0,
        }
    }
}
