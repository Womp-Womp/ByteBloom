#[cfg(test)]
mod plant_lifecycle_tests {
    use crate::engine::{self, new_game};
    use crate::plant::{LifeCycleStage};

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
        let result = sell_item(&mut inventory, &mut wallet, &market, "tomato", 5);
        assert!(result.is_ok());
        assert_eq!(inventory.get("tomato"), Some(&5));
        assert_eq!(wallet, 50.0);

        // Try to sell more tomatoes than available
        let result = sell_item(&mut inventory, &mut wallet, &market, "tomato", 10);
        assert!(result.is_err());
        assert_eq!(inventory.get("tomato"), Some(&5));
        assert_eq!(wallet, 50.0);

        // Try to sell an item that doesn't exist in the inventory
        let result = sell_item(&mut inventory, &mut wallet, &market, "potato", 5);
        assert!(result.is_err());

        // Try to sell an item that doesn't exist in the market
        let result = sell_item(&mut inventory, &mut wallet, &market, "cabbage", 5);
        assert!(result.is_err());
    }
}
