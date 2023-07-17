use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Country {
    Australia,
    Brazil,
    Japan,
    UnitedStates,
}