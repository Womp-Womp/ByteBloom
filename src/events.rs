use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameEvent {
    BlightSpotted,
    MarketCrash,
    BumperHarvest,
    PestInfestation(PestType),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PestType {
    Aphids,
    SpiderMites,
    Whiteflies,
}
