use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Soil {
    pub soil_type: SoilType,
    pub soil_moisture: f32,
    pub soil_nutrients: Nutrients,
    pub soil_ph: f32,
    pub weeds: f32,
}

#[derive(Serialize, Deserialize)]
pub enum SoilType {
    Sand,
    Clay,
    Loam,
}

#[derive(Serialize, Deserialize)]
pub struct Nutrients {
    pub nitrogen: f32,
    pub phosphorus: f32,
    pub potassium: f32,
}
use crate::plant;
#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub soil: Soil,
    pub plant: Option<plant::Plant>,
}

#[derive(Serialize, Deserialize)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
}

#[derive(Serialize, Deserialize)]
pub struct Plot {
    pub x: i32,
    pub y: i32,
    pub grid: Grid,
}

use crate::economy::Market;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct MainGameState {
    #[serde_as(as = "Vec<(_, _)>")]
    pub plots: HashMap<(i32, i32), Plot>,
    pub tick_counter: u64,
    pub inventory: HashMap<String, u32>,
    pub wallet: f32,
    pub market: Market,
}

use rand::Rng;

pub fn create_grid(width: u32, height: u32) -> Grid {
    let mut tiles = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(Tile {
                soil: Soil {
                    soil_type: SoilType::Loam,
                    soil_moisture: rng.gen_range(0.3..0.7),
                    soil_nutrients: Nutrients {
                        nitrogen: rng.gen_range(0.3..0.7),
                        phosphorus: rng.gen_range(0.3..0.7),
                        potassium: rng.gen_range(0.3..0.7),
                    },
                    soil_ph: rng.gen_range(6.0..7.5),
                    weeds: 0.0,
                },
                plant: None,
            });
        }
        tiles.push(row);
    }
    Grid { tiles }
}
