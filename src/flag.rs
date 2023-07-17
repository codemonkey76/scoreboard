use std::collections::HashMap;
use rand::Rng;
use crate::countries::Country;


#[derive(Debug)]
pub struct Flags(HashMap<Country, Flag>);

#[derive(Debug)]
pub struct Flag(&'static [u8]);

impl Flag {
    pub fn get(&self) -> &'static [u8] {
        self.0
    }
}

impl Flags {
    pub fn new() -> Self {
        let mut flags = HashMap::new();
        flags.insert(Country::Australia, Flag(include_bytes!("../assets/flags/au.svg")));
        flags.insert(Country::Brazil, Flag(include_bytes!("../assets/flags/br.svg")));
        flags.insert(Country::UnitedStates, Flag(include_bytes!("../assets/flags/us.svg")));

        Flags(flags)
    }

    pub fn get(&self, country: &Country) -> Option<&Flag> {
        self.0.get(country)
    }

    pub fn countries(&self) -> Vec<Country> {
        self.0.keys().cloned().collect()
    }

    pub fn random_country(&self) -> Option<&Country> {
        if self.0.is_empty() {
            return None;
        }
        let idx = rand::thread_rng().gen_range(0..self.0.len());
        self.0.keys().nth(idx)
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}