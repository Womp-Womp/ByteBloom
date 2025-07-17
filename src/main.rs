// src/main.rs

mod cli;
mod economy;
mod engine;
mod events;
mod garden;
mod pests;
mod plant;
mod plant_definitions;
mod saveload;
mod tui;
mod tests;
mod weather;

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
            let game_state = saveload::load_game("default_save.json").unwrap_or_else(|_| engine::new_game());
            // Then, perform the save operation.
            saveload::save_game(&game_state, filename).unwrap();
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
            saveload::load_game(filename).expect("Failed to load game from specified file.")
        }
        // For `Plant` or any other command, load the default game state.
        // If it doesn't exist, start a new game. This was the core logic from the `plant` branch.
        _ => saveload::load_game("default_save.json").unwrap_or_else(|_| {
            println!("No saved game found, starting a new one.");
            engine::new_game()
        }),
    };

    // Now, perform actions on the loaded or newly created game state.
    handle_command(args.command, &mut game_state);

    let tomato = plant::create_plant("tomato");
    println!("Created a plant: {}", tomato.species);

    let price = economy::get_market_price("corn");
    println!("The price of corn is: {}", price);

    // The TUI will take over the terminal, so we'll just call it and let it run.
    if let Err(e) = tui::draw_ui(&mut game_state) {
        println!("Error drawing UI: {}", e);
    }
}

fn handle_command(command: cli::Commands, game_state: &mut garden::MainGameState) {
    match command {
        cli::Commands::New | cli::Commands::Load { .. } | cli::Commands::Save { .. } => {
            // These are handled in the main function
        }
        cli::Commands::View { .. } => {
            if let Some(plot) = game_state.plots.get(&(0, 0)) {
                for row in &plot.grid.tiles {
                    for tile in row {
                        let symbol = match &tile.plant {
                            Some(plant) => match plant.life_cycle_stage {
                                plant::LifeCycleStage::Seed => 's',
                                plant::LifeCycleStage::Sprout => 'p',
                                plant::LifeCycleStage::Growing => 'P',
                                plant::LifeCycleStage::Mature => 'P',
                                plant::LifeCycleStage::Fruiting => 'P',
                                plant::LifeCycleStage::Withering => 'x',
                            },
                            None => '.',
                        };
                        print!("{} ", symbol);
                    }
                    println!();
                }
            }
        }
        cli::Commands::Plant { x, y, seed } => {
            engine::plant_seed(game_state, x, y, &seed);
        }
        cli::Commands::Water { x, y } => {
            if let Some(plot) = game_state.plots.get_mut(&(0, 0)) {
                if let Some(tile) = plot.grid.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
                    tile.soil.soil_moisture += 0.2;
                    tile.soil.soil_moisture = tile.soil.soil_moisture.clamp(0.0, 1.0);
                    println!("Watered tile ({}, {}). New moisture: {}", x, y, tile.soil.soil_moisture);
                } else {
                    println!("Invalid coordinates: ({}, {})", x, y);
                }
            }
        }
        cli::Commands::Fertilize { x, y, npk_mix } => {
            if let Some(plot) = game_state.plots.get_mut(&(0, 0)) {
                if let Some(tile) = plot.grid.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
                    // For simplicity, we'll parse a string like "0.1,0.1,0.1" for NPK values
                    let parts: Vec<Result<f32, _>> = npk_mix.split(',').map(|s| s.trim().parse()).collect();
                    if parts.len() == 3 && parts.iter().all(|p| p.is_ok()) {
                        let n = parts[0].as_ref().unwrap();
                        let p = parts[1].as_ref().unwrap();
                        let k = parts[2].as_ref().unwrap();

                        tile.soil.soil_nutrients.nitrogen += n;
                        tile.soil.soil_nutrients.phosphorus += p;
                        tile.soil.soil_nutrients.potassium += k;

                        // Clamp nutrient values
                        tile.soil.soil_nutrients.nitrogen = tile.soil.soil_nutrients.nitrogen.clamp(0.0, 1.0);
                        tile.soil.soil_nutrients.phosphorus = tile.soil.soil_nutrients.phosphorus.clamp(0.0, 1.0);
                        tile.soil.soil_nutrients.potassium = tile.soil.soil_nutrients.potassium.clamp(0.0, 1.0);

                        println!("Fertilized tile ({}, {}).", x, y);
                    } else {
                        println!("Invalid NPK mix format. Please use a format like '0.1,0.1,0.1'.");
                    }
                } else {
                    println!("Invalid coordinates: ({}, {})", x, y);
                }
            }
        }
        cli::Commands::Harvest { x, y } => {
            engine::harvest(game_state, x, y);
        }
        cli::Commands::Pesticide { x, y } => {
            engine::apply_pesticide(game_state, x, y);
        }
        cli::Commands::Forecast { ticks } => {
            engine::forecast(game_state, ticks);
        }
        cli::Commands::Market(market_command) => match market_command.command {
            cli::MarketCommands::Buy { item, quantity } => {
                match economy::buy_item(
                    &mut game_state.inventory,
                    &mut game_state.wallet,
                    &game_state.market,
                    &item,
                    quantity,
                ) {
                    Ok(()) => println!("Bought {} {}(s).", quantity, item),
                    Err(e) => println!("Error buying item: {}", e),
                }
            }
            cli::MarketCommands::Sell { item, quantity } => {
                match economy::sell_item(
                    &mut game_state.inventory,
                    &mut game_state.wallet,
                    &mut game_state.market,
                    &item,
                    quantity,
                ) {
                    Ok(()) => println!("Sold {} {}(s).", quantity, item),
                    Err(e) => println!("Error selling item: {}", e),
                }
            }
            cli::MarketCommands::View => {
                println!("{}", economy::view_market(&game_state.market));
            }
        },
    }
}
