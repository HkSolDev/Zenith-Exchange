# CEX-v2 Development Tasks

## Phase 1: Infrastructure Setup
- [x] Workspace initialized (`Cargo.toml`)
- [x] Kafka KRaft configuration implemented
- [x] Redis service added
- [ ] Postgres schema initialized (`init.sql` mounted and verified)
- [ ] `shared/domain` and `shared/db` crates compiled clean

## Phase 2: Kafka Foundation
- [ ] `shared/kafka` crate with `rdkafka`
- [ ] "Hello Kafka" Producer in `api-gateway`
- [ ] "Hello Kafka" Consumer in `matching-engine`

## Phase 3: API Gateway
- [ ] Order route accepts POST `/v1/order`
- [ ] JWT middleware for authentication
- [ ] Produce order to Kafka `orders.new`

---

## ⚡ Immediate Next Steps (Today)
1. **Fix `docker-compose.yml`**: Add `- ./init.sql:/docker-entrypoint-initdb.d/init.sql` to the `postgres` service.
2. **Reset Infrastructure**: Run `docker compose down -v && docker compose up -d`.
3. **Verify Tables**: Run `docker exec -it cex-v2-postgres-1 psql -U postgres -d app_db -c "\dt"`.
4. **Initialize `shared/db`**: Add `sqlx` and implement the connection pool logic.
5. **Phase 1 Blog**: Draft a post about setting up KRaft and Postgres init scripts.
