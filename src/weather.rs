use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Heatwave,
}
