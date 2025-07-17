use std::collections::HashMap;

pub struct Soil {
    pub soil_type: SoilType,
    pub moisture: f32,
    pub nutrients: Nutrients,
    pub ph: f32,
    pub weeds: f32,
}

pub enum SoilType {
    Sand,
    Clay,
    Loam,
}

pub struct Nutrients {
    pub nitrogen: f32,
    pub phosphorus: f32,
    pub potassium: f32,
}

pub struct Tile {
    pub soil: Soil,
}

pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
}

pub struct Plot {
    pub grid: Grid,
}

pub struct MainGameState {
    pub plots: HashMap<String, Plot>,
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
            });
        }
        tiles.push(row);
    }
    Grid { tiles }
}
