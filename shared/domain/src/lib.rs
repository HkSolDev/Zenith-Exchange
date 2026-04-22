use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::Display;

// TODO: Define Side enum (Buy/Sell)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell
}
// TODO: Define OrderType enum (Limit/Market)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
}
// TODO: Define Order struct 
// Fields needed: id (Uuid), user_id (String/Uuid), symbol (String), side, order_type, price (Decimal), qty (Decimal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
pub id: Uuid,
pub user_id: String,
pub symbol: String,
pub side: Side,
pub order_type: OrderType,
pub price: Decimal,
pub qty: Decimal,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
// TODO: Define Trade struct
// Fields needed: maker_order_id, taker_order_id, price, qty, timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub price: Decimal,
    pub qty: Decimal,
    pub timestamp: u64,
}
// IMPORTANT: Derive (Debug, Clone, Serialize, Deserialize) on ALL of them.
