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
        events: Vec::new(),
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

use crate::pests::{Pest, PestType};
use rand::seq::SliceRandom;

fn process_pests(state: &mut MainGameState) {
    let mut rng = rand::thread_rng();
    let mut pest_updates = Vec::new();
    let mut new_pests = Vec::new();

    for plot in state.plots.values() {
        for y in 0..plot.grid.tiles.len() {
            for x in 0..plot.grid.tiles[y].len() {
                if let Some(pest) = &plot.grid.tiles[y][x].pest {
                    let mut updated_pest = pest.clone();
                    updated_pest.infestation_level += 0.05;
                    pest_updates.push((x, y, updated_pest.clone()));

                    // Pest spreading
                    if rng.gen_bool(0.2) { // 20% chance to spread
                        let mut neighbors = Vec::new();
                        if x > 0 { neighbors.push((x - 1, y)); }
                        let (grid_width, grid_height) = (plot.grid.tiles[y].len(), plot.grid.tiles.len());
                        if x < grid_width - 1 { neighbors.push((x + 1, y)); }
                        if y > 0 { neighbors.push((x, y - 1)); }
                        if y < grid_height - 1 { neighbors.push((x, y + 1)); }

                        if let Some(&(nx, ny)) = neighbors.choose(&mut rng) {
                            if plot.grid.tiles[ny][nx].plant.is_some() && plot.grid.tiles[ny][nx].pest.is_none() {
                                new_pests.push((nx, ny, pest.clone()));
                            }
                        }
                    }
                } else if plot.grid.tiles[y][x].plant.is_some() {
                    if rng.gen_bool(0.1) { // 10% chance of pest appearing
                        let pest_type = match rng.gen_range(0..3) {
                            0 => PestType::Aphids,
                            1 => PestType::SpiderMites,
                            _ => PestType::Whiteflies,
                        };
                        new_pests.push((x, y, Pest {
                            pest_type: pest_type.clone(),
                            infestation_level: 0.1,
                        }));
                        println!("A pest has appeared: {:?} at ({}, {})", pest_type, x, y);
                    }
                }
            }
        }
    }

    for plot in state.plots.values_mut() {
        for (x, y, pest) in &pest_updates {
            if let Some(plant) = &mut plot.grid.tiles[*y][*x].plant {
                plant.health -= pest.infestation_level * 0.1;
                println!("Pest at ({}, {}) is damaging the plant. Plant health: {}", x, y, plant.health);
            }
            plot.grid.tiles[*y][*x].pest = Some(pest.clone());
        }
        for (x, y, pest) in &new_pests {
            plot.grid.tiles[*y][*x].pest = Some(pest.clone());
            println!("Pest has spread to ({}, {})", x, y);
        }
    }
}

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
    process_pests(state);

    economy::update_market_prices(&mut state.market);
}

pub fn apply_pesticide(game_state: &mut MainGameState, x: u32, y: u32) {
    if let Some(plot) = game_state.plots.get_mut(&(0, 0)) {
        if let Some(tile) = plot.grid.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            if tile.pest.is_some() {
                tile.pest = None;
                println!("Applied pesticide to tile ({}, {})", x, y);
            } else {
                println!("No pest to remove at ({}, {})", x, y);
            }
        } else {
            println!("Invalid coordinates: ({}, {})", x, y);
        }
    }
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
