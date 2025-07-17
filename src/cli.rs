// src/cli.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Starts a new game
    New,
    /// Saves the game state
    Save {
        /// The filename to save the game to
        filename: String,
    },
    /// Loads the game state
    Load {
        /// The filename to load the game from
        filename: String,
    },
    /// Views the garden
    View {
        #[clap(long)]
        from: String,
        #[clap(long)]
        to: String,
    },
    /// Plants a seed
    Plant {
        x: u32,
        y: u32,
        #[clap(long)]
        seed: String,
    },
    /// Waters a tile
    Water {
        x: u32,
        y: u32,
    },
    /// Harvests a mature plant
    Harvest {
        x: u32,
        y: u32,
    },
    /// Shows market prices
    Market,
}

pub fn parse_args() -> Args {
    Args::parse()
}
