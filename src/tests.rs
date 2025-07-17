#[cfg(test)]
mod tests {
    use crate::engine::{self, new_game};
    use crate::plant::{self, LifeCycleStage};

    #[test]
    fn test_harvest() {
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
}
