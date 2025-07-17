// src/economy.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct Market {
    pub items: HashMap<String, f32>,
    pub supply_demand: HashMap<String, f32>,
}

impl Default for Market {
    fn default() -> Self {
        let mut items = HashMap::new();
        items.insert("tomato".to_string(), 10.0);
        items.insert("potato".to_string(), 5.0);
        items.insert("corn".to_string(), 15.0);

        Market {
            items,
            supply_demand: HashMap::new(),
        }
    }
}

pub fn sell_item(
    inventory: &mut HashMap<String, u32>,
    wallet: &mut f32,
    market: &mut Market,
    item_name: &str,
    quantity: u32,
) -> Result<(), &'static str> {
    if let Some(price) = market.items.get(item_name) {
        if let Some(available_quantity) = inventory.get_mut(item_name) {
            if *available_quantity >= quantity {
                *available_quantity -= quantity;
                *wallet += price * quantity as f32;

                let supply_demand_effect = market.supply_demand.entry(item_name.to_string()).or_insert(1.0);
                *supply_demand_effect -= (quantity as f32) / 100.0;

                Ok(())
            } else {
                Err("Not enough items to sell.")
            }
        } else {
            Err("Item not found in inventory.")
        }
    } else {
        Err("Item not found in market.")
    }
}

pub fn update_market_prices(market: &mut Market) {
    for (item, price) in market.items.iter_mut() {
        let change = rand::thread_rng().gen_range(-0.05..0.05);
        let supply_demand_effect = market.supply_demand.get(item).cloned().unwrap_or(1.0);
        *price *= 1.0 + change + supply_demand_effect - 1.0;

        if *price < 0.1 {
            *price = 0.1;
        }

        if let Some(supply_demand_effect) = market.supply_demand.get_mut(item) {
            *supply_demand_effect += 0.005;
            if *supply_demand_effect > 1.0 {
                *supply_demand_effect = 1.0;
            }
        }
    }
}

pub fn buy_item(
    inventory: &mut HashMap<String, u32>,
    wallet: &mut f32,
    market: &Market,
    item_name: &str,
    quantity: u32,
) -> Result<(), &'static str> {
    if let Some(price) = market.items.get(item_name) {
        let cost = price * quantity as f32;
        if *wallet >= cost {
            *wallet -= cost;
            let entry = inventory.entry(item_name.to_string()).or_insert(0);
            *entry += quantity;
            Ok(())
        } else {
            Err("Not enough cash to buy.")
        }
    } else {
        Err("Item not found in market.")
    }
}

pub fn get_market_price(_item: &str) -> f32 {
    // Placeholder for market price logic
    10.0
}

pub fn view_market(market: &Market) -> String {
    let mut market_view = String::from("Item\t\tPrice\n");
    for (item, price) in &market.items {
        market_view.push_str(&format!("{}\t\t{}\n", item, price));
    }
    market_view
}

