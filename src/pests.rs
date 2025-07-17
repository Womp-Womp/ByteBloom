use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pest {
    pub pest_type: PestType,
    pub infestation_level: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PestType {
    Aphids,
    SpiderMites,
    Whiteflies,
}
