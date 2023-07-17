use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::Instant;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Matches {
    pub matches: Vec<BjjMatch>,
}

impl Default for Matches {
    fn default() -> Self {
        Self {
            matches: BjjMatch::sample_matches(),
        }
    }
}

#[derive(Debug)]
pub struct MatchInformation {
    pub competitor_one: CompetitorPoints,
    pub competitor_two: CompetitorPoints,
    pub time_limit_minutes: usize,
    last_started: Option<Instant>,
    // elapsed_milliseconds: u128,
    time_remaining_milliseconds: u128,
    pub match_state: MatchState,
}

impl MatchInformation {
    pub fn new(time: usize) -> Self {
        Self {
            competitor_one: CompetitorPoints {
                points: 0,
                advantages: 0,
                penalties: 0,
            },
            competitor_two: CompetitorPoints {
                points: 0,
                advantages: 0,
                penalties: 0,
            },
            time_limit_minutes: time,
            last_started: None,
            time_remaining_milliseconds: (time * 60 * 1000) as u128,
            match_state: MatchState::NotStarted,
        }
    }

    pub fn start(&mut self) {
        self.last_started = Some(Instant::now());
        self.match_state = MatchState::InProgress;
    }

    pub fn pause(&mut self) {
        if self.match_state == MatchState::InProgress {
            self.match_state = MatchState::Paused;
            if let Some(started) = self.last_started {
                self.time_remaining_milliseconds -= started.elapsed().as_millis();
            }
        }
    }

    pub fn stop(&mut self) {}

    pub fn get_time_remaining(&self) -> u128 {
        match self.match_state {
            MatchState::InProgress => {
                if let Some(started) = self.last_started {
                    self.time_remaining_milliseconds - started.elapsed().as_millis()
                } else {
                    self.time_remaining_milliseconds
                }
            }
            _ => self.time_remaining_milliseconds,
        }
    }

    pub fn get_formatted_remaining_time(&self) -> String {
        let remaining = self.get_time_remaining();

        let minutes = remaining / 60000;
        let seconds = (remaining % 60000) / 1000;
        let milliseconds = remaining % 1000;

        format!("{}:{:02}.{:03}", minutes, seconds, milliseconds)
    }



    pub fn set_time(&mut self, time: usize) {
        self.time_limit_minutes = time;
        self.time_remaining_milliseconds = (time * 60 * 1000) as u128;
    }

    pub fn reset(&mut self) {
        self.competitor_one.reset();
        self.competitor_two.reset();
        self.time_remaining_milliseconds = (self.time_limit_minutes * 60 * 1000) as u128;
        self.match_state = MatchState::NotStarted;
    }
}

#[derive(Debug, PartialEq)]
pub enum MatchState {
    NotStarted,
    InProgress,
    Paused,
    Finished,
}

#[derive(Debug)]
pub enum PointType {
    Advantage,
    Penalty,
    Point,
}

#[derive(Debug)]
pub struct CompetitorPoints {
    pub points: usize,
    pub advantages: usize,
    pub penalties: usize,
}

impl CompetitorPoints {
    pub fn reset(&mut self) {
        self.points = 0;
        self.advantages = 0;
        self.penalties = 0;
    }
    pub fn add(&mut self, point_type: PointType, amount: usize) {
        match point_type {
            PointType::Advantage => {
                self.advantages += amount;
            }
            PointType::Penalty => {
                self.penalties += amount;
            }
            PointType::Point => {
                self.points += amount;
            }
        }
    }
    pub fn subtract(&mut self, point_type: PointType) {
        match point_type {
            PointType::Advantage => {
                if self.advantages > 0 {
                    self.advantages -= 1;
                }
            }
            PointType::Penalty => {
                if self.penalties > 0 {
                    self.penalties -= 1;
                }
            }
            PointType::Point => {
                if self.points > 0 {
                    self.points -= 1;
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BjjMatch {
    pub competitor_one: Competitor,
    pub competitor_two: Competitor,
    pub time_limit: usize,
    pub fight_info: String,
    pub fight_sub_info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Competitor {
    pub first_name: String,
    pub last_name: String,
    pub team: String,
    pub country: Country,
}
impl Competitor {
    pub fn display_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
            .trim()
            .to_owned()
    }
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
        let divs = [
            "Male GI / Purple / Master 1 (30+) / -70 KG (Feather)",
            "Male GI / Black / Adult / Above 100.5 KG+ (Ultra Heavy)",
            "Male Absolute GI / White / Master 1 (30+) / Open Weight",
            "Female GI / Blue / Adult / -69 KG (Middle)",
        ];

        let types = [
            "Heat",
            "Quarter Final",
            "Semi Final",
            "Final",
        ];

        let mut rng = rand::thread_rng();
        let div = divs[rng.gen_range(0..divs.len())].to_owned();
        let fight_type = types[rng.gen_range(0..types.len())].to_owned();
        Self {
            competitor_one: Competitor::default(),
            competitor_two: Competitor::default(),
            time_limit: 5,
            fight_info: fight_type,
            fight_sub_info: div,
        }
    }
}

impl BjjMatch {
    pub fn sample_matches() -> Vec<BjjMatch> {
        vec![
            BjjMatch::default(),
            BjjMatch::default(),
            BjjMatch::default(),
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
        let teams = [
            "Alliance",
            "Caza BJJ",
            "Fight Club Jiu-Jitsu",
            "Gracie Barra",
            "Carlson Gracie",
            "Infinity",
            "One Purpose BJJ",
            "Atos"
        ];

        let mut rng = rand::thread_rng();
        let team = teams[rng.gen_range(0..teams.len())].to_owned();
        Self {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            team,
            country: Country::UnitedStates,
        }
    }
}
