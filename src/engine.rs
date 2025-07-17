// src/engine.rs

use crate::garden::{create_grid, MainGameState, Plot};
use crate::plant;
use std::collections::HashMap;
use std::fs;
use std::io;

pub fn new_game() -> MainGameState {
    let mut plots = HashMap::new();
    let initial_plot = Plot {
        x: 0,
        y: 0,
        grid: create_grid(10, 10),
    };
    plots.insert((0, 0), initial_plot);

    MainGameState { plots }
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

pub fn save_game(game_state: &MainGameState) -> io::Result<()> {
    // In a real game, you'd serialize the game state to a file.
    // For now, we'll just pretend to save.
    fs::write("savegame.dat", "This is a fake save file.")?;
    Ok(())
}

pub fn load_game() -> io::Result<MainGameState> {
    // In a real game, you'd deserialize the game state from a file.
    // For now, we'll always return an error to start a new game.
    Err(io::Error::new(io::ErrorKind::NotFound, "No save file found"))
}


pub fn init_game() -> MainGameState {
    new_game()
}

pub fn run_game_tick(state: &mut MainGameState) {
    // Placeholder for game tick logic
}
