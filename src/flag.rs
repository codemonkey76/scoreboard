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
        flags.insert(Country::UnitedArabEmirates, Flag(include_bytes!("../assets/flags/ae.svg")));
        flags.insert(Country::Argentina, Flag(include_bytes!("../assets/flags/ar.svg")));
        flags.insert(Country::Australia, Flag(include_bytes!("../assets/flags/au.svg")));
        flags.insert(Country::Belgium, Flag(include_bytes!("../assets/flags/be.svg")));
        flags.insert(Country::Brazil, Flag(include_bytes!("../assets/flags/br.svg")));
        flags.insert(Country::Canada, Flag(include_bytes!("../assets/flags/ca.svg")));
        flags.insert(Country::Germany, Flag(include_bytes!("../assets/flags/de.svg")));
        flags.insert(Country::Denmark, Flag(include_bytes!("../assets/flags/dk.svg")));
        flags.insert(Country::Spain, Flag(include_bytes!("../assets/flags/es.svg")));
        flags.insert(Country::Finland, Flag(include_bytes!("../assets/flags/fi.svg")));
        flags.insert(Country::Ireland, Flag(include_bytes!("../assets/flags/ie.svg")));
        flags.insert(Country::Morocco, Flag(include_bytes!("../assets/flags/ma.svg")));
        flags.insert(Country::Norway, Flag(include_bytes!("../assets/flags/no.svg")));
        flags.insert(Country::Philippines, Flag(include_bytes!("../assets/flags/ph.svg")));
        flags.insert(Country::NewZealand, Flag(include_bytes!("../assets/flags/nz.svg")));
        flags.insert(Country::Japan, Flag(include_bytes!("../assets/flags/jp.svg")));
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