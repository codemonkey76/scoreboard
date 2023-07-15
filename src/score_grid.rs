use egui_multiwin::egui::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ScoreGrid {
    pub window: Rect,

    pub top: Rect,
    pub middle: Rect,
    pub bottom: Rect,

    pub competitor_one_flag: Rect,
    pub competitor_one_name: Rect,
    pub competitor_one_score: Rect,
    pub competitor_one_advantage: Rect,
    pub competitor_one_penalty: Rect,
    pub competitor_one_team: Rect,

    pub competitor_two_flag: Rect,
    pub competitor_two_name: Rect,
    pub competitor_two_score: Rect,
    pub competitor_two_advantage: Rect,
    pub competitor_two_penalty: Rect,
    pub competitor_two_team: Rect,

    pub timer: Rect,
    pub fight_info: Rect,
    pub fight_sub_info: Rect,
    pub logo: Rect,
}

impl ScoreGrid {
    pub fn calc_grids(window: Rect, config: &GridConfig) -> Self {
        let (top, bottom) = window.split_top_bottom_at_fraction(1.0 - config.competitor_time);
        let (top, middle) = top.split_top_bottom_at_fraction(0.5);

        let (competitor_one, competitor_one_score) =
            top.split_left_right_at_fraction(1.0 - config.points);
        let (competitor_one_left, competitor_one_right) =
            competitor_one.split_left_right_at_fraction(1.0 - config.name_adv);
        let (competitor_one_advantage, competitor_one_penalty) =
            competitor_one_right.split_top_bottom_at_fraction(0.5);
        let (competitor_one_row_1, competitor_one_team) =
            competitor_one_left.split_top_bottom_at_fraction(config.name_team);
        let (competitor_one_flag, competitor_one_name) =
            competitor_one_row_1.split_left_right_at_fraction(config.flag_name);

        let (competitor_two, competitor_two_score) =
            middle.split_left_right_at_fraction(1.0 - config.points);
        let (competitor_two_left, competitor_two_right) =
            competitor_two.split_left_right_at_fraction(1.0 - config.name_adv);
        let (competitor_two_advantage, competitor_two_penalty) =
            competitor_two_right.split_top_bottom_at_fraction(0.5);
        let (competitor_two_row_1, competitor_two_team) =
            competitor_two_left.split_top_bottom_at_fraction(config.name_team);
        let (competitor_two_flag, competitor_two_name) =
            competitor_two_row_1.split_left_right_at_fraction(config.flag_name);

        let (timer_left, logo) = bottom.split_left_right_at_fraction(1.0 - config.points);
        let (timer, fight_info_full) =
            timer_left.split_left_right_at_fraction(config.timer_fight_info);
        let (fight_info, fight_sub_info) =
            fight_info_full.split_top_bottom_at_fraction(config.fight_info_sub_info);

        Self {
            window,
            top,
            middle,
            bottom,
            competitor_one_flag,
            competitor_one_name,
            competitor_one_score,
            competitor_one_advantage,
            competitor_one_penalty,
            competitor_one_team,
            competitor_two_flag,
            competitor_two_name,
            competitor_two_score,
            competitor_two_advantage,
            competitor_two_penalty,
            competitor_two_team,
            timer,
            fight_info,
            fight_sub_info,
            logo,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridConfig {
    /// Horizontal percentage of screen taken up by points on the right
    pub points: f32,

    /// Horizontal percentage of competitor area taken up by advantages/penalties
    pub name_adv: f32,

    /// Vertical percentage of screen taken up by the timer
    pub competitor_time: f32,

    /// Horizontal percentage taken up by flag in relation to competitor name
    pub flag_name: f32,

    /// Vertical percentage taken up by Competitor name in relation to Team name
    pub name_team: f32,

    /// Horizontal percentage taken up by the timer in relation to fight info
    pub timer_fight_info: f32,

    /// Vertical percentage taken up by the fight info in relation to the fight sub info
    pub fight_info_sub_info: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            points: 0.2,
            name_adv: 0.5,
            competitor_time: 0.3,
            flag_name: 0.2,
            name_team: 0.5,
            timer_fight_info: 0.5,
            fight_info_sub_info: 0.5,
        }
    }
}
