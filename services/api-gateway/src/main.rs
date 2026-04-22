use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use actix_web::{App, HttpResponse, HttpServer, web, post};
use domain::Order;
//use serde_json::json;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/order")]
async fn get_order(order: web::Json<Order>) -> HttpResponse {
   let order = order.into_inner();
   println!("order: {:#?}", order);
   
   HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  println!("Produce the message the kafka");
  let producer: FutureProducer = ClientConfig::new()
  .set("bootstrap.servers", "localhost:9092")
  .set("message.timeout.ms", "5000")
  .create()
  .expect("Failed to create producer");


  println!("Api start"); 
    // use the route as is a simpler route so i think its better to use than the overhead of services
    HttpServer::new(move|| App::new()
    // scope is for routing
    .app_data(producer.clone())
    .service(web::scope("/v1").service(get_order))
    .route("/health", web::get().to(health)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}