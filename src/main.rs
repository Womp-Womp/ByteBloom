// src/main.rs

mod cli;
mod economy;
mod engine;
mod garden;
mod plant;
mod tui;
mod saveload;

use std::fs;
use std::fs::File;
use std::io::Write;
fn main() {
    println!("Hello from ByteBloom Gardens!");

    let args = cli::parse_args();
    println!("Command-line arguments: {:?}", args);

    let mut game_state = match args.command {
        cli::Commands::New => engine::new_game(),
        cli::Commands::Save { filename } => {
            let mut game_state = load_game("game_state.json").unwrap_or_else(|_| engine::new_game());
            save_game(&game_state, &filename).unwrap();
            println!("Game saved to {}", filename);
            return;
        }
        cli::Commands::Load { filename } => {
            let game_state = load_game(&filename).unwrap();
            println!("Game loaded from {}", filename);
            // You might want to do something with the loaded game state here
            return;
        }
        _ => engine::new_game(),
    };

    let tomato = plant::create_plant("tomato");
    println!("Created a plant: {}", tomato.species);

    let price = economy::get_market_price("corn");
    println!("The price of corn is: {}", price);

    // The TUI will take over the terminal, so we'll just call it and let it run.
    // Note: This will fail to compile until we add the TUI dependencies.
    // if let Err(e) = tui::draw_ui() {
    //     println!("Error drawing UI: {}", e);
    // }
}

pub fn save_game(game_state: &garden::MainGameState, filename: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string(game_state).unwrap();
    let mut file = File::create(filename)?;
    file.write_all( serialized.as_bytes())?;
    Ok(())
}

pub fn load_game(filename: &str) -> std::io::Result<garden::MainGameState> {
    let data = fs::read_to_string(filename)?;
    let game_state = serde_json::from_str(&data).unwrap();
    Ok(game_state)
}
