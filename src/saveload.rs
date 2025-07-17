#[cfg(test)]
mod tests {
    use crate::engine;
    use crate::save_game;
    use crate::load_game;

    #[test]
    fn test_save_and_load() {
        let game_state = engine::new_game();
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
