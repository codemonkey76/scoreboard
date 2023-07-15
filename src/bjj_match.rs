use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Matches {
    pub matches: Vec<BjjMatch>,
}

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
    pub team: String,
    pub country: Country,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Country {
    Australia,
    Brazil,
    Japan,
    UnitedStates,
}

impl Default for BjjMatch {
    fn default() -> Self {
        Self {
            competitor_one: Competitor::default(),
            competitor_two: Competitor::default(),
            time_limit: 5,
            fight_type: String::from("Semi-Final"),
            division_name: String::from("Middleweight <82.3KG"),
        }
    }
}

impl BjjMatch {
    pub fn sample_matches() -> Vec<BjjMatch> {
        vec![
            BjjMatch {
                competitor_one: Competitor {
                    first_name: String::from("John"),
                    last_name: String::from("Smith"),
                    team: String::from("Gracie Barra"),
                    country: Country::Brazil,
                },
                ..Default::default()
            },
            BjjMatch::default(),
            BjjMatch::default(),
            BjjMatch::default(),
        ]
    }
}

impl Display for Competitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)?;

        if !self.team.is_empty() {
            write!(f, " ({})", self.team)?;
        }

        Ok(())
    }
}
impl Default for Competitor {
    fn default() -> Self {
        Self {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            team: "".to_owned(),
            country: Country::UnitedStates,
        }
    }
}
