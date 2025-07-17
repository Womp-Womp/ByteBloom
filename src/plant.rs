// src/plant.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plant {
    pub species: String,
    pub genetics: PlantGenetics,
    pub life_cycle_stage: LifeCycleStage,
    pub age: u32,
    pub maturity_age: u32,
    pub wither_time: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PlantGenetics {
    pub growth_time: u32,
    pub yield_range: (u32, u32),
    pub water_req: f32,
    pub nutrient_drain: (f32, f32, f32),
    pub light_req: f32,
    pub pest_resistance: f32,
    pub disease_resistance: f32,
    pub genetic_stability: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum LifeCycleStage {
    Seed,
    Sprout,
    Growing,
    Mature,
    Fruiting,
    Withering,
}

pub fn create_plant(species: &str) -> Plant {
    // Placeholder for plant creation
    Plant {
        species: species.to_string(),
        genetics: PlantGenetics {
            growth_time: 10,
            yield_range: (1, 5),
            water_req: 5.0,
            nutrient_drain: (0.1, 0.1, 0.1),
            light_req: 5.0,
            pest_resistance: 0.1,
            disease_resistance: 0.1,
            genetic_stability: 0.9,
        },
        life_cycle_stage: LifeCycleStage::Seed,
        age: 0,
        maturity_age: 10,
        wither_time: 15,
    }
}
