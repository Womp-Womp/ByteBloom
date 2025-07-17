// src/engine.rs

use crate::garden::{create_grid, MainGameState, Plot};
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
    }
}

pub fn init_game() -> MainGameState {
    new_game()
}

pub fn run_game_tick(state: &mut MainGameState) {
    // Placeholder for game tick logic
}
