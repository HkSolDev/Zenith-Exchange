// Shared Kafka helpers
use rdkafka::config::ClientConfig;
/// This for the consumer side
use rdkafka::consumer::StreamConsumer;
///This for the producer side
use rdkafka::producer::FutureProducer;

pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Error in creating the producer")
}

pub fn create_consumer(brokers: &str, group_id: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id) //We need group id for the service so each service have uniques id
        .set("enable.auto.commit", "true") // so this will allow when the result come it automatcally use the result and move forward
        .set("auto.offset.reset", "earliest") // If the service start for the first time its say here is everything to consume from the beginning
        .create()
        .expect("Error in creating the consumer")
}
