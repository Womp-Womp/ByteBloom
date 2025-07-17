#[cfg(test)]
mod tests {
    use crate::engine::{new_game, plant_seed, run_game_tick};
    use crate::garden::{Grid, MainGameState, Nutrients, Soil, SoilType, Tile};
    use crate::plant::{LifeCycleStage};

    fn setup_test_game() -> MainGameState {
        let mut game = new_game();
        // Clear default plots and set up a predictable grid
        game.plots.clear();
        let mut tiles = Vec::new();
        for _ in 0..10 {
            let mut row = Vec::new();
            for _ in 0..10 {
                row.push(Tile {
                    soil: Soil {
                        soil_type: SoilType::Loam,
                        soil_moisture: 0.5,
                        soil_nutrients: Nutrients {
                            nitrogen: 0.5,
                            phosphorus: 0.5,
                            potassium: 0.5,
                        },
                        soil_ph: 7.0,
                        weeds: 0.0,
                    },
                    plant: None,
                });
            }
            tiles.push(row);
        }
        game.plots.insert(
            (0, 0),
            crate::garden::Plot {
                x: 0,
                y: 0,
                grid: Grid { tiles },
            },
        );
        game
    }

    #[test]
    fn test_plant_growth_ideal_conditions() {
        let mut game = setup_test_game();
        plant_seed(&mut game, 0, 0, "tomato");

        let initial_age = game.plots[&(0, 0)].grid.tiles[0][0]
            .plant
            .as_ref()
            .unwrap()
            .age;
        assert_eq!(initial_age, 0, "Plant should have an initial age of 0.");

        run_game_tick(&mut game);

        let new_age = game.plots[&(0, 0)].grid.tiles[0][0]
            .plant
            .as_ref()
            .unwrap()
            .age;
        assert_eq!(
            new_age, 1,
            "Plant should age by 1 under ideal soil conditions."
        );
    }
    #[test]
    fn test_watering_tile() {
        let mut game = setup_test_game();
        let initial_moisture = game.plots[&(0, 0)].grid.tiles[0][0].soil.soil_moisture;

        // Simulate watering the tile
        if let Some(plot) = game.plots.get_mut(&(0, 0)) {
            if let Some(tile) = plot.grid.tiles.get_mut(0).and_then(|row| row.get_mut(0)) {
                tile.soil.soil_moisture += 0.2;
                tile.soil.soil_moisture = tile.soil.soil_moisture.clamp(0.0, 1.0);
            }
        }

        let new_moisture = game.plots[&(0, 0)].grid.tiles[0][0].soil.soil_moisture;
        assert!(
            new_moisture > initial_moisture,
            "Watering should increase soil moisture."
        );
        assert!(
            new_moisture <= 1.0,
            "Soil moisture should not exceed 1.0."
        );
    }

    #[test]
    fn test_fertilizing_tile() {
        let mut game = setup_test_game();
        let initial_nutrients = &game.plots[&(0, 0)].grid.tiles[0][0].soil.soil_nutrients;
        let initial_n = initial_nutrients.nitrogen;
        let initial_p = initial_nutrients.phosphorus;
        let initial_k = initial_nutrients.potassium;

        // Simulate fertilizing the tile
        if let Some(plot) = game.plots.get_mut(&(0, 0)) {
            if let Some(tile) = plot.grid.tiles.get_mut(0).and_then(|row| row.get_mut(0)) {
                tile.soil.soil_nutrients.nitrogen += 0.1;
                tile.soil.soil_nutrients.phosphorus += 0.1;
                tile.soil.soil_nutrients.potassium += 0.1;
            }
        }

        let new_nutrients = &game.plots[&(0, 0)].grid.tiles[0][0].soil.soil_nutrients;
        assert!(
            new_nutrients.nitrogen > initial_n,
            "Fertilizing should increase nitrogen."
        );
        assert!(
            new_nutrients.phosphorus > initial_p,
            "Fertilizing should increase phosphorus."
        );
        assert!(
            new_nutrients.potassium > initial_k,
            "Fertilizing should increase potassium."
        );
    }

    #[test]
    fn test_plant_growth_non_ideal_moisture() {
        let mut game = setup_test_game();
        plant_seed(&mut game, 0, 0, "tomato");

        // Set moisture to a non-ideal level
        if let Some(plot) = game.plots.get_mut(&(0, 0)) {
            if let Some(tile) = plot.grid.tiles.get_mut(0).and_then(|row| row.get_mut(0)) {
                tile.soil.soil_moisture = 0.1; // Below the ideal range of 0.4-0.6
            }
        }

        run_game_tick(&mut game);

        let new_age = game.plots[&(0, 0)].grid.tiles[0][0]
            .plant
            .as_ref()
            .unwrap()
            .age;
        assert_eq!(
            new_age, 0,
            "Plant should not age as fast under non-ideal moisture conditions."
        );
    }
}
