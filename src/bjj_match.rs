use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BjjMatch {
    pub competitor_one: Competitor,
    pub competitor_two: Competitor,
    pub time_limit: usize,
    pub fight_type: String,
    pub division_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Competitor {
    pub first_name: String,
    pub last_name: String,
    pub team: Option<Team>,
    pub country: Country
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub logo: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Country {
    Australia,
    Brazil,
    Japan,
    UnitedStates
}