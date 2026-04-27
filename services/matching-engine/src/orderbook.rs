use std::collections::BTreeMap;
use domain::{Order, Side};
use rust_decimal::Decimal;

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
    pub fn add_order(&mut self, order: Order) {
        let order_clone = order.clone();
        match order_clone.side{
           Side::Buy => {
            self.bids.entry(order.price).or_insert_with(Vec::new).push(order)
           },
           Side::Sell => {
            self.asks.entry(order.price).or_insert_with(Vec::new).push(order)
           }
        }
       
    }
     fn match_order(&mut self,order:Order){
        let order_clone = order.clone();

        match order_clone.side {
            Side::Buy =>{
                while let Some(mut entry) = self.asks.first_entry() {
        
                    let best_price = entry.key();
                    if order.price >= *best_price{
             
                       let orders_at_price = entry.get_mut();

                       while !orders_at_price.is_empty() && order.qty > Decimal::ZERO {

                        let matching_order = &mut orders_at_price[0];

                        let fill_amount =  order.qty.min(matching_order.qty);

                        order.qty -= fill_amount;
                        matching_order.qty -= fill_amount;

                        //If matching_order is empty, remove it from the list
                        if matching_order.qty == Decimal::ZERO{
                            orders_at_price.remove(0);
                        }

                        if orders_at_price.qty == Decimal::ZERO{
                         entry.remove();
                        }
                    }
                       }
                    }
                }

                
                       
                
            Side::Sell => {
                while let Some(mut entry) = self.bids.last_entry() {
                    let best_price = entry.key();

                    if *best_price >= order.price {
                        let orders_at_price = entry.get_mut();
                        
                        while orders_at_price.len() > 1 && order.qty > Decimal::ZERO {
                            
                           let matching_order = &mut orders_at_price[0];
                           let fill_amount = order.qty.min(matching_order.qty);
                           order.qty -= fill_amount;
                           matching_order.qty -= fill_amount;
                            
                            if orders_at_price.is_empty(){
                                entry.remove();
                            }
                            if matching_order.qty == Decimal::ZERO{
                                 orders_at_price.remove(0);
                            }
                            if order.qty == Decimal::ZERO{
                                 break;
                            }
                        }

                    }else {
                        break;
                    }}
            } 
        }
        }
}