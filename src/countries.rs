use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Country {
    UnitedArabEmirates,
    Argentina,
    Australia,
    Belgium,
    Brazil,
    Canada,
    Germany,
    Denmark,
    Spain,
    Finland,
    France,
    Ireland,
    Morocco,
    Norway,
    Philippines,
    NewZealand,
    Japan,
    UnitedStates,
}