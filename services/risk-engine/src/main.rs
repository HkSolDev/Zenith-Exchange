use db::{init_db_pool, DbPool};
use domain::Order;
use kafka::create_consumer;
use rdkafka::{
    consumer::{self, Consumer, StreamConsumer},
    Message,
};
use serde_json::Result;
use::rust

pub mod checks;

#[tokio::main]
async fn main() {
    println!("Risk Engine starting...");

    let broker = "localhost:9092";

    let consumer = create_consumer(broker, "risk_engine");

    let db_pool = init_db_pool().await.expect("Failed to connect to the database")

    consumer
        .subscribe(&["order_news"])
        .expect("Risk Engine is not able to subsribe to the topic");

    println!("The risk_engine is live and listening for orders");

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                if let Some(Ok(text)) = msg.payload_view::<str>() {
                    let order_res: Result<Order, serde_json::Error> = serde_json::from_str(text);

                    match order_res {
                        Ok(order) => {
                            println!("Order come in the risk Engine");

                            if order.qty <= Decimal::ZERO || order.price <= Decimal::ZERO {
                                println!(
                                    "PRice of the order {} or the qty {} is not valid",
                                    order.id, order.qty
                                );
                                continue;
                            }
                        }
                        Err(_) => {
                            println!("The order receive is not a valid type {}", order);
                            continue;
                        }
                    }
                }
            }
            Err(_) => println!("Error in receiving the orders through the kafa in the risk_engine"),
        }
    }
}
