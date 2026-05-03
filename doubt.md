## Why use the rustls, chrono, uuid did the sqlx not provide this then what exactly i provide
sqlx = { version = "0.8", features = [ "runtime-tokio-rustls", "postgres", "uuid", "decimal", "chrono" ] }

**Answer:** Even though `sqlx` has features for `uuid` and `chrono`, those features only enable `sqlx` to *understand* those types (e.g., mapping a Postgres `UUID` column to a Rust `uuid::Uuid` struct). You still need to include the actual crates (`uuid`, `chrono`) in your `Cargo.toml` so you can use those types in your own code (like creating a new UUID or getting the current time).

## Why we need Type Aliases
Type Aliases: Using `pub type DbPool = PgPool;`

**Answer:** This is about **encapsulation**. If we decide to switch from `sqlx` to another library later, or if we want to add more metadata to our pool, we only change it in one place (`shared/db/src/lib.rs`). Every other service just uses `DbPool` without caring that it's actually an `sqlx::PgPool`.

## What is ProtoBuf ???
**Answer:** Protocol Buffers (ProtoBuf) is a method of serializing structured data (like JSON) but into a **binary format**. It's much smaller and faster to parse than JSON. In a CEX, we might use it for internal communication between microservices where every microsecond counts.

## Database Connection Pooling
```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await?;
```
**Answer:** This creates a queue/pool of 5 active connections. If 10 requests come at the same time, 5 will use the connections immediately, and the other 5 will wait until one becomes free. This prevents overwhelming the database.

## How to implement the DLQ (Dead Letter Queue)
**Answer:** In Kafka, a DLQ is just another topic (e.g., `orders.failed`). 
1. Your consumer tries to process a message.
2. If it fails (e.g., bad JSON), you catch the error.
3. You `produce` that same message to the `orders.failed` topic.
4. You `commit` the original message so the consumer moves to the next one.
This prevents a "poison message" from blocking your entire engine.

## When to use `expect()` vs `match` vs `?`
**Answer:** In a high-performance system like an exchange, there is a very strict rule for this:

1. **Use `expect()` or `unwrap()` ONLY at startup.**
   - Example: Connecting to the database, parsing the `.env` file, or subscribing to Kafka.
   - *Why?* If the DB is down when the app starts, it's better to crash immediately so Kubernetes/Docker can restart it.
2. **Use `match` or `?` everywhere else (The "Hot Path").**
   - Example: Parsing an incoming order from a user, or sending a trade to Kafka in a loop.
   - *Why?* If a user sends a badly formatted JSON order, you **do not** want your entire matching engine to crash. You want to `match` the error, log it, send it to a DLQ, and keep processing the next orders!