// src/engine.rs

use crate::garden::{create_grid, MainGameState, Plot};
use crate::plant;
use std::collections::HashMap;

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

pub fn init_game() -> MainGameState {
    new_game()
}

pub fn run_game_tick(state: &mut MainGameState) {
    state.tick_counter += 1;

    for plot in state.plots.values_mut() {
        for row in plot.grid.tiles.iter_mut() {
            for tile in row.iter_mut() {
                if let Some(plant) = &mut tile.plant {
                    plant.age += 1;
                    if plant.age >= plant.maturity_age {
                        plant.life_cycle_stage = plant::LifeCycleStage::Mature;
                    }
                }
            }
        }
    }
}
