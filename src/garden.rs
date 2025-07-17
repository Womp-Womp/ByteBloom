use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Soil {
    pub soil_type: SoilType,
    pub moisture: f32,
    pub nutrients: Nutrients,
    pub ph: f32,
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

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct MainGameState {
    #[serde_as(as = "Vec<(_, _)>")]
    pub plots: HashMap<(i32, i32), Plot>,
    pub tick_counter: u64,
}

pub fn create_grid(width: u32, height: u32) -> Grid {
    let mut tiles = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(Tile {
                soil: Soil {
                    soil_type: SoilType::Loam,
                    moisture: 0.5,
                    nutrients: Nutrients {
                        nitrogen: 0.5,
                        phosphorus: 0.5,
                        potassium: 0.5,
                    },
                    ph: 7.0,
                    weeds: 0.0,
                },
                plant: None,
            });
        }
        tiles.push(row);
    }
    Grid { tiles }
}
