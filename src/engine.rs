// src/engine.rs

use crate::garden::{create_grid, MainGameState, Plot};
use std::collections::HashMap;

pub fn init_game() -> MainGameState {
    let mut plots = HashMap::new();
    let initial_plot = Plot {
        grid: create_grid(10, 10),
    };
    plots.insert("main_plot".to_string(), initial_plot);

    MainGameState {
        plots,
    }
}

pub fn run_game_tick(state: &mut MainGameState) {
    // Placeholder for game tick logic
}
