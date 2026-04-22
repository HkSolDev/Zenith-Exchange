use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub enum Order {
 id: Uuid,
 user_id: String,
 symbol: String,
 side: Side,
 order_type: OrderType,
 price: Decimal,
 qty: Decimal,
}
// TODO: Define Trade struct
// Fields needed: maker_order_id, taker_order_id, price, qty, timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeStruct {
    maker_order_id: Uuid,
    taker_order_id: Uuid,
    price: Decimal,
    qty: Decimal,
    timestamp: u64,
}
// IMPORTANT: Derive (Debug, Clone, Serialize, Deserialize) on ALL of them.
