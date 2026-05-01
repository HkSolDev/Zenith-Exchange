use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::FutureRecord;
use std::time::Duration;

pub mod orderbook;
use domain::Order;
use kafka::{create_consumer, create_producer};
use orderbook::Orderbook;

#[tokio::main] // Using tokio::main instead of actix_web::main if we don't need HTTP yet
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Matching Engine starting...");

    // Create the Consumer
    let brokers = "localhost:9092";
    let consumer: StreamConsumer = create_consumer(brokers, "matching-engine-group");

    // Subscribe the Consumer
    consumer
        .subscribe(&["orders.validated"]) // Matching engine listens to validated orders
        .expect("Failed to subscribe");

    // Create a Producer for DLQ and Trade events
    let producer = create_producer(brokers);

    let mut orderbook = Orderbook::new();

    println!("Waiting for orders...");
    loop {
        match consumer.recv().await {
            Ok(msg) => {
                if let Some(Ok(text)) = msg.payload_view::<str>() {
                    let order_res: Result<Order, serde_json::Error> = serde_json::from_str(text);

                    match order_res {
                        Ok(order) => {
                            println!("Received order: {:?}", order.id);
                            let trades = orderbook.add_order(order);
                            
                            for trade in trades {
                                if let Ok(trade_json) = serde_json::to_string(&trade) {
                                    let _ = producer.send(
                                        FutureRecord::to("trades.executed")
                                            .payload(&trade_json)
                                            .key(&trade.id.to_string()),
                                        Duration::from_secs(0)
                                    ).await;
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to deserialize order: {}", e);
                            let _ = producer
                                .send(
                                    FutureRecord::to("orders.failed")
                                        .payload(text)
                                        .key(msg.key().unwrap_or(b"")),
                                    Duration::from_secs(0),
                                )
                                .await;
                        }
                    }
                }
            }
            Err(e) => println!("Kafka error: {}", e),
        }
    }
}
