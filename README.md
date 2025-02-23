
### Setup
- Create `.env` file with contents `WEBSOCKET_ENDPOINT=wss://mainnet.infura.io/ws/v3/<YOUR-PROJECT-KEY>` (add key).
In general, ws connection could come from any source / provider
- `cargo run`. It will install dependencies and run the app.
- `cargo run --bin generate_fixtures` (for tests)

### Tests
Check relevant unit & integration tests inside `src/blocks_handler.rs` & in `tests/events_handler_test.rs`. Run `Cargo test` to run both unit & integration tests. 

