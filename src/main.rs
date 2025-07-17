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

    let mut game_state = match args.command {
        cli::Commands::New => engine::new_game(),
        _ => panic!("Not implemented yet"),
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
