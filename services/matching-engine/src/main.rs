use rdkafka::config::ClientConfig;
use rdkafka::consumer::{self, Consumer, StreamConsumer};
use rdkafka::message::Message;

pub mod orderbook;
use domain::Order;

use kafka::create_consumer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Matching Engine starting...");

    //Create the Consumer
    let consumer: StreamConsumer = create_consumer("localhost:9092", "Matching-engine-group");

    // Subscribe the Consumer
    consumer
        .subscribe(&["test-topic"])
        .expect("Failed to subscribe");

    let producer = create_producer("localhost:9092");

    let orderbook = Orderbook::new();
    // Infinite Async Loop
    loop {
        let message = consumer.recv().await;
        match message {
            Ok(msg) => {
                if let Some(Ok(text)) = msg.payload_view::<str>() { //the payload_view is a zero copy it just creates a Reference to the data that is setting inside the kafka buffer
                    let order: Result<Order, serde_json::Error> = serde_json::from_str(text);

                    match order {
                        Ok(order) => orderbook.add_order(order),
                        Err(e) => {println!("Failed to deserialize order: {}", e);

                     producer.send(FutureRecord::to("order-failed").payload(text).key(&text.to_string()), Duration::from_secs(0)).await.expect("failed to send message");
                        
                    
                    }
                    }

                }
            }
            Err(e) => println!("Kafka error: {}", e),
        }
    }

    Ok(())
}
