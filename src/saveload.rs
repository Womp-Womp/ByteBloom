use crate::garden;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn save_game(game_state: &garden::MainGameState, filename: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string(game_state).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn load_game(filename: &str) -> std::io::Result<garden::MainGameState> {
    let data = fs::read_to_string(filename)?;
    let game_state = serde_json::from_str(&data).unwrap();
    Ok(game_state)
}

#[cfg(test)]
mod tests {
    use crate::engine;
    use crate::saveload::{save_game, load_game};

    #[test]
    fn test_save_and_load() {
        let mut game_state = engine::new_game();
        engine::plant_seed(&mut game_state, 0, 0, "tomato");
        let filename = "test_game.json";

        // Save the game
        save_game(&game_state, filename).unwrap();

        // Load the game
        let loaded_game_state = load_game(filename).unwrap();

        // Verify that the loaded game state is the same as the original
        assert_eq!(
            game_state.plots.keys().collect::<Vec<_>>(),
            loaded_game_state.plots.keys().collect::<Vec<_>>()
        );

        // Clean up the test file
        std::fs::remove_file(filename).unwrap();
    }
}
