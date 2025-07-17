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
    // Handle commands that are one-off actions and might exit early.
    // We match on a reference to `args.command` so we can use it again later.
    match &args.command {
        // The Save command from `main` branch loads the default game, saves it to a new file, and exits.
        cli::Commands::Save { filename } => {
            // First, get the current game state to save. We load the default save file, or create a new game if none exists.
            let game_state = engine::load_game().unwrap_or_else(|_| engine::new_game());
            // Then, perform the save operation.
            engine::save_game(&game_state, filename).unwrap();
            println!("Game saved to {}", filename);
            return; // Exit after saving, as was the behavior in the `main` branch.
        }
        // The Load command is now handled in the main state initialization below.
        _ => { /* Continue to main logic */ }
    }

    // Determine the initial game state based on the command, or by loading the default.
    let mut game_state = match &args.command {
        cli::Commands::New => {
            println!("Starting a new game.");
            engine::new_game()
        }
        cli::Commands::Load { filename } => {
            println!("Loading game from {}.", filename);
            engine::load_game(filename)
                .expect("Failed to load game from specified file.")
        }
        // For `Plant` or any other command, load the default game state.
        // If it doesn't exist, start a new game. This was the core logic from the `plant` branch.
        _ => engine::load_game().unwrap_or_else(|_| {
            println!("No saved game found, starting a new one.");
            engine::new_game()
        }),
    };

    // Now, perform actions on the loaded or newly created game state.
    match args.command {
        // `New`, `Load`, and `Save` were handled above.
        // `Save` has an early return, so it won't reach this match block.
        // `New` and `Load` simply set the initial state, so no further action is needed here.
        cli::Commands::New | cli::Commands::Load { .. } => { /* State already initialized */ }

        // The `Plant` command from the feature branch modifies the state.
        cli::Commands::Plant { x, y, seed } => {
            engine::plant_seed(&mut game_state, x, y, &seed);
            // Consider saving the game here automatically if that's desired.
            // engine::save_game(&game_state, "default_save.json").unwrap();
        }

        // All other commands fall through. `Save` is already handled.
        _ => { /* Potentially add a game loop or other logic here */ }
    }
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
