#[cfg(test)]
mod plant_lifecycle_tests {
    use crate::engine::{self, new_game};
    use crate::plant::{LifeCycleStage};

    #[test]
    fn test_full_lifecycle_and_harvest() {
        let mut game_state = new_game();
        engine::plant_seed(&mut game_state, 0, 0, "tomato");

        // Age the plant to maturity
        for _ in 0..10 {
            engine::run_game_tick(&mut game_state);
        }

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert_eq!(tile.plant.as_ref().unwrap().life_cycle_stage, LifeCycleStage::Mature);

        // Harvest the mature plant
        engine::harvest(&mut game_state, 0, 0);
        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert!(tile.plant.is_none());
        assert!(*game_state.inventory.get("tomato").unwrap() > 0);

        // Plant another seed and let it wither
        engine::plant_seed(&mut game_state, 0, 0, "tomato");
        for _ in 0..15 {
            engine::run_game_tick(&mut game_state);
        }

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert_eq!(tile.plant.as_ref().unwrap().life_cycle_stage, LifeCycleStage::Withering);

        // Try to harvest the withered plant
        let inventory_before_harvest = game_state.inventory.clone();
        engine::harvest(&mut game_state, 0, 0);
        assert_eq!(game_state.inventory, inventory_before_harvest);
    }

    #[test]
    fn test_harvest_mature_plant() {
        let mut game_state = new_game();
        engine::plant_seed(&mut game_state, 0, 0, "tomato");

        // Age the plant to maturity
        for _ in 0..10 {
            engine::run_game_tick(&mut game_state);
        }

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert_eq!(tile.plant.as_ref().unwrap().life_cycle_stage, LifeCycleStage::Mature);

        engine::harvest(&mut game_state, 0, 0);

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert!(tile.plant.is_none());
        assert!(*game_state.inventory.get("tomato").unwrap() > 0);
    }

    #[test]
    fn test_run_game_tick() {
        let mut game_state = new_game();
        engine::plant_seed(&mut game_state, 0, 0, "tomato");

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert_eq!(tile.plant.as_ref().unwrap().age, 0);
        assert_eq!(tile.plant.as_ref().unwrap().life_cycle_stage, LifeCycleStage::Seed);

        engine::run_game_tick(&mut game_state);

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert_eq!(tile.plant.as_ref().unwrap().age, 1);
        assert_eq!(tile.plant.as_ref().unwrap().life_cycle_stage, LifeCycleStage::Seed);

        for _ in 0..9 {
            engine::run_game_tick(&mut game_state);
        }

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert_eq!(tile.plant.as_ref().unwrap().age, 10);
        assert_eq!(tile.plant.as_ref().unwrap().life_cycle_stage, LifeCycleStage::Mature);
    }

    #[test]
    fn test_harvest_immature_plant() {
        let mut game_state = new_game();
        engine::plant_seed(&mut game_state, 0, 0, "tomato");

        engine::harvest(&mut game_state, 0, 0);

        let plot = game_state.plots.get(&(0, 0)).unwrap();
        let tile = &plot.grid.tiles[0][0];
        assert!(tile.plant.is_some());
        assert_eq!(game_state.inventory.get("tomato"), None);
    }

    use crate::economy::{sell_item, Market};
    use std::collections::HashMap;

    #[test]
    fn test_sell_item() {
        let mut inventory = HashMap::new();
        inventory.insert("tomato".to_string(), 10);
        let mut wallet = 0.0;
        let mut market = Market::default();
        market.items.insert("tomato".to_string(), 10.0);

        // Sell 5 tomatoes
        let result = sell_item(&mut inventory, &mut wallet, &mut market, "tomato", 5);
        assert!(result.is_ok());
        assert_eq!(inventory.get("tomato"), Some(&5));
        assert_eq!(wallet, 50.0);

        // Try to sell more tomatoes than available
        let result = sell_item(&mut inventory, &mut wallet, &mut market, "tomato", 10);
        assert!(result.is_err());
        assert_eq!(inventory.get("tomato"), Some(&5));
        assert_eq!(wallet, 50.0);

        // Try to sell an item that doesn't exist in the inventory
        let result = sell_item(&mut inventory, &mut wallet, &mut market, "potato", 5);
        assert!(result.is_err());

        // Try to sell an item that doesn't exist in the market
        let result = sell_item(&mut inventory, &mut wallet, &mut market, "cabbage", 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_economic_loop() {
        let mut game_state = new_game();
        let initial_wallet = game_state.wallet;

        // Plant a seed
        engine::plant_seed(&mut game_state, 0, 0, "tomato");

        // Age the plant to maturity
        for _ in 0..10 {
            engine::run_game_tick(&mut game_state);
        }

        // Harvest the plant
        engine::harvest(&mut game_state, 0, 0);
        let harvested_amount = *game_state.inventory.get("tomato").unwrap();
        assert!(harvested_amount > 0);

        // Sell the harvested item
        let result = sell_item(
            &mut game_state.inventory,
            &mut game_state.wallet,
            &mut game_state.market,
            "tomato",
            harvested_amount,
        );
        assert!(result.is_ok());
        assert_eq!(game_state.inventory.get("tomato"), Some(&0));
        assert!(game_state.wallet > initial_wallet);
    }
}
