// src/engine.rs

use crate::garden::{create_grid, MainGameState, Plot};
use crate::plant;
use crate::weather::Weather;
use std::collections::HashMap;
use crate::economy::Market;

pub fn new_game() -> MainGameState {
    let mut plots = HashMap::new();
    let initial_plot = Plot {
        x: 0,
        y: 0,
        grid: create_grid(10, 10),
    };
    plots.insert((0, 0), initial_plot);


    MainGameState {
        plots,
        tick_counter: 0,
        inventory: HashMap::new(),
        wallet: 100.0,
        market: Market::default(),
        current_weather: Weather::Sunny,
    }
}

pub fn plant_seed(game_state: &mut MainGameState, x: u32, y: u32, seed: &str) {
    if let Some(plot) = game_state.plots.get_mut(&(0, 0)) {
        if let Some(tile) = plot.grid.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            if tile.plant.is_none() {
                let new_plant = plant::create_plant(seed);
                tile.plant = Some(new_plant);
                println!("Planted a {} at ({}, {})", seed, x, y);
            } else {
                println!("There is already a plant at ({}, {})", x, y);
            }
        } else {
            println!("Invalid coordinates: ({}, {})", x, y);
        }
    }
}

use rand::Rng;

pub fn harvest(game_state: &mut MainGameState, x: u32, y: u32) {
    if let Some(plot) = game_state.plots.get_mut(&(0, 0)) {
        if let Some(tile) = plot.grid.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            if let Some(plant) = &tile.plant {
                if plant.life_cycle_stage == plant::LifeCycleStage::Fruiting {
                    let yield_amount = rand::thread_rng().gen_range(plant.genetics.yield_range.0..=plant.genetics.yield_range.1);
                    println!("Harvested {} of {} from ({}, {})", yield_amount, plant.species, x, y);
                    let entry = game_state.inventory.entry(plant.species.clone()).or_insert(0);
                    *entry += yield_amount;
                    tile.plant = None;
                } else {
                    println!("The plant at ({}, {}) is not ready to be harvested.", x, y);
                }
            } else {
                println!("There is no plant at ({}, {})", x, y);
            }
        } else {
            println!("Invalid coordinates: ({}, {})", x, y);
        }
    }
}

use crate::economy;

use rand::seq::SliceRandom;

fn process_weather(state: &mut MainGameState) {
    let mut rng = rand::thread_rng();
    let next_weather = [
        Weather::Sunny,
        Weather::Cloudy,
        Weather::Rainy,
        Weather::Heatwave,
    ]
    .choose(&mut rng)
    .unwrap();
    state.current_weather = *next_weather;
    println!("Weather updated to: {:?}", state.current_weather);
}

fn process_plants(state: &mut MainGameState) {
    for plot in state.plots.values_mut() {
        for row in plot.grid.tiles.iter_mut() {
            for tile in row.iter_mut() {
                if let Some(plant) = &mut tile.plant {
                    let mut growth_rate = 1.0;

                    if state.current_weather == Weather::Heatwave {
                        growth_rate *= 0.5; // 50% growth reduction during heatwave
                    }

                    // Check moisture levels
                    let (min_moisture, max_moisture) = plant.genetics.ideal_moisture_range;
                    if tile.soil.soil_moisture < min_moisture || tile.soil.soil_moisture > max_moisture {
                        growth_rate *= 0.8; // 20% growth reduction if outside ideal moisture
                    }

                    plant.growth_progress += growth_rate;

                    if plant.growth_progress >= 1.0 {
                        plant.age += 1;
                        plant.growth_progress -= 1.0;
                    }

                    if plant.age >= plant.wither_time {
                        plant.life_cycle_stage = plant::LifeCycleStage::Withering;
                    } else if plant.age >= plant.maturity_age {
                        plant.life_cycle_stage = plant::LifeCycleStage::Fruiting;
                    } else if plant.age >= plant.maturity_age / 2 {
                        plant.life_cycle_stage = plant::LifeCycleStage::Growing;
                    } else if plant.age > 0 {
                        plant.life_cycle_stage = plant::LifeCycleStage::Sprout;
                    }
                }
            }
        }
    }
}

fn process_environment(state: &mut MainGameState) {
    for plot in state.plots.values_mut() {
        for row in plot.grid.tiles.iter_mut() {
            for tile in row.iter_mut() {
                match state.current_weather {
                    Weather::Sunny => tile.soil.soil_moisture -= 0.05,
                    Weather::Rainy => tile.soil.soil_moisture += 0.2,
                    Weather::Heatwave => tile.soil.soil_moisture -= 0.1,
                    _ => {}
                }

                if let Some(plant) = &mut tile.plant {
                    // Consume water and nutrients
                    tile.soil.soil_moisture -= 0.01; // Constant water consumption for now
                    tile.soil.soil_nutrients.nitrogen -= plant.genetics.nutrient_consumption.0;
                    tile.soil.soil_nutrients.phosphorus -= plant.genetics.nutrient_consumption.1;
                    tile.soil.soil_nutrients.potassium -= plant.genetics.nutrient_consumption.2;
                }

                // Clamp soil values
                tile.soil.soil_moisture = tile.soil.soil_moisture.clamp(0.0, 1.0);
                tile.soil.soil_nutrients.nitrogen = tile.soil.soil_nutrients.nitrogen.clamp(0.0, 1.0);
                tile.soil.soil_nutrients.phosphorus = tile.soil.soil_nutrients.phosphorus.clamp(0.0, 1.0);
                tile.soil.soil_nutrients.potassium = tile.soil.soil_nutrients.potassium.clamp(0.0, 1.0);
            }
        }
    }
}

pub fn run_game_tick(state: &mut MainGameState, weather: Option<Weather>) {
    state.tick_counter += 1;

    if let Some(weather) = weather {
        state.current_weather = weather;
    } else {
        process_weather(state);
    }
    process_environment(state);
    process_plants(state);

    economy::update_market_prices(&mut state.market);
}

pub fn forecast(game_state: &MainGameState, ticks: u64) {
    let mut rng = rand::thread_rng();
    let mut weather;
    println!("Weather forecast:");
    for i in 0..ticks {
        let next_weather = [
            Weather::Sunny,
            Weather::Cloudy,
            Weather::Rainy,
            Weather::Heatwave,
        ]
        .choose(&mut rng)
        .unwrap();
        weather = *next_weather;
        println!("Tick {}: {:?}", game_state.tick_counter + i + 1, weather);
    }
}
