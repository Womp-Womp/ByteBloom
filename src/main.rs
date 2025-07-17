// src/main.rs

mod cli;
mod economy;
mod engine;
mod garden;
mod plant;
mod tui;

fn main() {
    println!("Hello from ByteBloom Gardens!");

    let args = cli::parse_args();
    println!("Command-line arguments: {:?}", args);

    let mut game_state = engine::init_game();
    println!("Game state initialized with time: {}", game_state.game_time);

    engine::run_game_tick(&mut game_state);
    println!("Game state after one tick: {}", game_state.game_time);

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
