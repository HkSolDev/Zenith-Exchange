use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

// TODO: Define Side enum (Buy/Sell)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}
// TODO: Define OrderType enum (Limit/Market)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
}
// TODO: Define Order struct
// Fields needed: id (Uuid), user_id (String/Uuid), symbol (String), side, order_type, price (Decimal), qty (Decimal)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Balance {
    pub user_id: Uuid,
    pub symbol: String,
    pub amount: Decimal,
    pub free: Decimal,
    pub locked: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSide {
    Maker,
    Taker,
}

// Fields needed: maker_order_id, taker_order_id, price, qty, timestamp
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Trade {
    pub id: Uuid,
    pub symbol: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub created_at: DateTime<Utc>,
}
// IMPORTANT: Derive (Debug, Clone, Serialize, Deserialize) on ALL of them.
