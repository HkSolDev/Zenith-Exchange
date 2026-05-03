# CEX-v2 Development Tasks

## Phase 1: Infrastructure Setup
- [x] Workspace initialized (`Cargo.toml`)
- [x] Kafka KRaft configuration implemented
- [x] Redis service added
- [x] Postgres schema initialized (`init.sql` mounted and verified)
- [x] `shared/domain` and `shared/db` crates compiled clean

## Phase 2: Kafka Foundation
- [x] `shared/kafka` crate with `rdkafka`
- [x] "Hello Kafka" Producer in `api-gateway`
- [x] "Hello Kafka" Consumer in `matching-engine`

## Phase 3: Risk Engine (Pre-match checks)
- [ ] Kafka consumer listening to `orders.new`
- [ ] Validate order (Check if user has enough balance - requires DB read)
- [ ] Publish to `orders.validated` if pass
- [ ] Publish to `orders.failed` (DLQ) if fail

## Phase 4: Matching Engine
- [x] BTreeMap Orderbook (Bids/Asks)
- [x] Price-Time Priority matching logic
- [x] Return `Vec<Trade>` from matching loop
- [x] Kafka consumer loop for `orders.validated`
- [x] Publish `Trade` events to `trades.executed`

## Phase 5: Wallet Service (Accounts & Settlement)
- [ ] Kafka consumer listening to `trades.executed`
- [ ] Connect to PostgreSQL using `sqlx`
- [ ] Atomic DB transactions to update user balances
- [ ] Handle locked vs free funds

## Phase 6: Market Data & Analytics
- [ ] Kafka consumer listening to `trades.executed`
- [ ] Aggregate trades into OHLCV candles
- [ ] Calculate 24h rolling stats

## Phase 7: API Gateway & WebSocket Feeds
- [ ] Actix-web server setup
- [ ] `POST /v1/order` REST endpoint -> publishes to `orders.new`
- [ ] JWT authentication middleware
- [ ] WebSocket upgrade route
- [ ] Subscribe to Redis Pub/Sub for real-time market data


## 📝 Documentation & Blogs (To be completed after code)
- [ ] **Phase 1 Blog**: "How I structured a Rust microservices workspace from scratch"
    - *Hint: Why choose a workspace instead of one giant crate? Explain the benefits of isolation and independent scaling.*
- [ ] **ADR-001**: Architecture Decision Record for Kafka Spine
- [ ] **Phase 2-4 Blog**: "Building the Guard & the Brain: Risk & Matching Engines"

