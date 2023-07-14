use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use fake::Fake;
use fake::faker::name::raw::*;
use fake::locales::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BjjMatch {
    pub competitor_one: Competitor,
    pub competitor_two: Competitor,
    pub time_limit: usize,
    pub fight_type: String,
    pub division_name: String,
}

impl Default for BjjMatch {
    fn default() -> Self {
        Self {
            competitor_one: Competitor::default(),
            competitor_two: Competitor::default(),
            time_limit: 5,
            fight_type: String::from("Semi-Final"),
            division_name: String::from("Middleweight <82.3KG")
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
                    team: Some(Team {
                        name: String::from("Gracie Barra"),
                        logo: None
                    }),
                    country: Country::Brazil
                },
              ..Default::default()
            },
            BjjMatch::default(), BjjMatch::default(), BjjMatch::default()]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Competitor {
    pub first_name: String,
    pub last_name: String,
    pub team: Option<Team>,
    pub country: Country
}

impl Display for Competitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)?;

        if let Some(team) = &self.team {
            write!(f, " ({})", team.name)?;
        }

        Ok(())
    }
}
impl Default for Competitor {
    fn default() -> Self {
        Self {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            team: None,
            country: Country::UnitedStates
        }
    }
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