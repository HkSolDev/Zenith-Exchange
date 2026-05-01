use std::time::Duration;

use rdkafka::producer::{FutureProducer, FutureRecord};

use actix_web::{http::header::ContentType, post, web, App, HttpResponse, HttpServer};
use domain::Order;

use kafka::create_producer;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/order")]
async fn get_order(order: web::Json<Order>, producer: web::Data<FutureProducer>) -> HttpResponse {
    let order = order.into_inner();

    println!("order: {:#?}", order);

    let serialize_order = serde_json::to_string(&order).expect("Failed to serialize the order");

    producer
        .send(
            FutureRecord::to("test-topic")
                .payload(&serialize_order)
                .key(&order.id.to_string()),
            Duration::from_secs(0),
        )
        .await
        .expect("Failed to send the order to the Mathing engine");

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Order Received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Produce the message the kafka");

    let producer = create_producer("localhost:9092");

    producer
        .send(
            FutureRecord::to("test-topic").payload("hello CEX").key(""),
            Duration::from_secs(0),
        )
        .await
        .expect("Failed to send message To Kafka");

    println!("Api start");
    // use the route as is a simpler route so i think its better to use than the overhead of services
    HttpServer::new(move || {
        App::new()
            // scope is for routing
            .app_data(web::Data::new(producer.clone()))
            .service(web::scope("/v1").service(get_order))
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
