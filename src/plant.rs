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
    pub growth_progress: f32,
    pub health: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PlantGenetics {
    pub growth_time: u32,
    pub yield_range: (u32, u32),
    pub ideal_moisture_range: (f32, f32),
    pub nutrient_consumption: (f32, f32, f32),
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

use crate::plant_definitions::PLANTS;
use rand::seq::SliceRandom;

pub fn create_plant(species: &str) -> Plant {
    PLANTS
        .iter()
        .find(|p| p.species == species)
        .map(|p| {
            let mut plant = Plant {
                species: p.species.clone(),
                genetics: PlantGenetics {
                    growth_time: p.genetics.growth_time,
                    yield_range: p.genetics.yield_range,
                    ideal_moisture_range: p.genetics.ideal_moisture_range,
                    nutrient_consumption: p.genetics.nutrient_consumption,
                    light_req: p.genetics.light_req,
                    pest_resistance: p.genetics.pest_resistance,
                    disease_resistance: p.genetics.disease_resistance,
                    genetic_stability: p.genetics.genetic_stability,
                },
                life_cycle_stage: LifeCycleStage::Seed,
                age: 0,
                maturity_age: p.maturity_age,
                wither_time: p.wither_time,
                growth_progress: 0.0,
                health: 1.0,
            };
            plant
        })
        .unwrap_or_else(|| {
            // Fallback to a random plant if species not found
            let mut rng = rand::thread_rng();
            let random_plant = PLANTS.choose(&mut rng).unwrap();
            let mut plant = Plant {
                species: random_plant.species.clone(),
                genetics: PlantGenetics {
                    growth_time: random_plant.genetics.growth_time,
                    yield_range: random_plant.genetics.yield_range,
                    ideal_moisture_range: random_plant.genetics.ideal_moisture_range,
                    nutrient_consumption: random_plant.genetics.nutrient_consumption,
                    light_req: random_plant.genetics.light_req,
                    pest_resistance: random_plant.genetics.pest_resistance,
                    disease_resistance: random_plant.genetics.disease_resistance,
                    genetic_stability: random_plant.genetics.genetic_stability,
                },
                life_cycle_stage: LifeCycleStage::Seed,
                age: 0,
                maturity_age: random_plant.maturity_age,
                wither_time: random_plant.wither_time,
                growth_progress: 0.0,
                health: 1.0,
            };
            plant
        })
}
