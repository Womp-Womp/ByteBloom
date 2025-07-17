// src/economy.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Market {
    pub items: HashMap<String, f32>,
}

pub fn sell_item(
    inventory: &mut HashMap<String, u32>,
    wallet: &mut f32,
    market: &Market,
    item_name: &str,
    quantity: u32,
) -> Result<(), &'static str> {
    if let Some(price) = market.items.get(item_name) {
        if let Some(available_quantity) = inventory.get_mut(item_name) {
            if *available_quantity >= quantity {
                *available_quantity -= quantity;
                *wallet += price * quantity as f32;
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

pub fn get_market_price(_item: &str) -> f32 {
    // Placeholder for market price logic
    10.0
}

pub fn buy_stock(_ticker: &str, _amount: u32) {
    // Placeholder for stock buying logic
}

pub fn sell_stock(_ticker: &str, _amount: u32) {
    // Placeholder for stock selling logic
}
