use chrono::Utc;
use domain::{Order, Side, Trade};
use rust_decimal::Decimal;
use std::collections::BTreeMap;

pub struct Orderbook {
    pub bids: BTreeMap<Decimal, Vec<Order>>,
    pub asks: BTreeMap<Decimal, Vec<Order>>,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, mut order: Order) -> Vec<Trade> {
        // 1. Try to match the order first
        let trades = self.match_order(&mut order);

        // 2. If there is still quantity left, add it to the book
        if order.qty > Decimal::ZERO {
            match order.side {
                Side::Buy => self
                    .bids
                    .entry(order.price)
                    .or_insert_with(Vec::new)
                    .push(order),
                Side::Sell => self
                    .asks
                    .entry(order.price)
                    .or_insert_with(Vec::new)
                    .push(order),
            }
        }
        trades
    }

    fn match_order(&mut self, order: &mut Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        match order.side {
            Side::Buy => {
                while order.qty > Decimal::ZERO {
                    let mut entry = match self.asks.first_entry() {
                        Some(e) => e,
                        None => break, // No more asks to match against
                    };

                    let best_ask_price = *entry.key();
                    if order.price < best_ask_price {
                        break; // Price doesn't match
                    }

                    let orders_at_price = entry.get_mut();
                    while !orders_at_price.is_empty() && order.qty > Decimal::ZERO {
                        let matching_order = &mut orders_at_price[0];
                        let fill_amount = order.qty.min(matching_order.qty);

                        order.qty -= fill_amount;
                        matching_order.qty -= fill_amount;

                        let trade = Trade {
                            id: order.id,
                            price: matching_order.price,
                            qty: fill_amount,
                            symbol: order.symbol.clone(),
                            maker_order_id: matching_order.id, // MAKER IS THE SELLER WHO IS IN THE ORDERBOOK
                            taker_order_id: order.id, // TAKE WHO COME AND MATHCING IT ORDER INTO THE ORDER BOOK
                            created_at: Utc::now(),
                        };

                        trades.push(trade);

                        if matching_order.qty == Decimal::ZERO {
                            orders_at_price.remove(0);
                        }
                    }

                    if orders_at_price.is_empty() {
                        entry.remove();
                    }
                }
            }
            Side::Sell => {
                while order.qty > Decimal::ZERO {
                    let mut entry = match self.bids.last_entry() {
                        Some(e) => e,
                        None => break, // No more bids to match against
                    };

                    let best_bid_price = *entry.key();
                    if order.price > best_bid_price {
                        break; // Price doesn't match
                    }

                    let orders_at_price = entry.get_mut();
                    while !orders_at_price.is_empty() && order.qty > Decimal::ZERO {
                        let matching_order = &mut orders_at_price[0];
                        let fill_amount = order.qty.min(matching_order.qty);

                        order.qty -= fill_amount;
                        matching_order.qty -= fill_amount;

                        let trade = Trade {
                            id: order.id,
                            price: matching_order.price,
                            qty: fill_amount,
                            symbol: order.symbol.clone(),
                            maker_order_id: matching_order.id, // MAKER IS THE SELLER WHO IS IN THE ORDERBOOK
                            taker_order_id: order.id, // TAKE WHO COME AND MATHCING IT ORDER INTO THE ORDER BOOK
                            created_at: Utc::now(),
                        };
                        trades.push(trade);

                        if matching_order.qty == Decimal::ZERO {
                            orders_at_price.remove(0);
                        }
                    }

                    if orders_at_price.is_empty() {
                        entry.remove();
                    }
                }
            }
        }
        trades
    }
}
