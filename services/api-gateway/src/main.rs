use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use actix_web::{App, HttpResponse, HttpServer, web, post};
use domain::Order;


async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/order")]
async fn get_order(order: web::Json<Order>, producer:web::Data<FutureProducer>) -> HttpResponse {
   let order = order.into_inner();
   // TODO: Fix the rdkafka::util::Timeout error here. 
   // The timeout argument `0` is an integer, but it needs to be a `std::time::Duration`.
   // Try using `std::time::Duration::from_secs(0)`.
  //  let result = producer.send(FutureRecord::to("orders")
  //   .payload(&order.to_string()), 0);

    
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