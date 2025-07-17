// src/engine.rs

use crate::garden::Grid;
use crate::economy::Market;

pub struct GameState {
    pub grid: Grid,
    pub market: Market,
    pub game_time: u64,
}

pub fn init_game() -> GameState {
    // Placeholder for game initialization
    GameState {
        grid: crate::garden::create_grid(100, 100),
        market: Market {},
        game_time: 0,
    }
}

pub fn run_game_tick(state: &mut GameState) {
    // Placeholder for game tick logic
    state.game_time += 1;
}
