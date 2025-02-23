
### Setup
- Create `.env` file with contents `WEBSOCKET_ENDPOINT=wss://mainnet.infura.io/ws/v3/<YOUR-PROJECT-KEY>` (add key).
In general, ws connection could come from any source / provider
- `cargo run`. It will install dependencies and run the app.
- `cargo run --bin generate_fixtures` (for tests)

### Tests
Check relevant unit & integration tests inside `src/blocks_handler.rs` & in `tests/events_handler_test.rs`. Run `Cargo test` to run both unit & integration tests. 


Example program output
```aiignore
[2025-02-23T20:09:01Z INFO  uniswap_dai_usd_monitor] 🚀 App launched. Fetching blocks...
[2025-02-23T20:09:02Z DEBUG web3::transports::ws] [1] Calling: {"jsonrpc":"2.0","method":"eth_subscribe","params":["newHeads"],"id":1}
[2025-02-23T20:09:12Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911313, hash: 0x4058daba514eb098522a9d0145a081503917291e641cd93fe2c6b5bddcfe0c6e
[2025-02-23T20:09:24Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911314, hash: 0x17bbd935e972b510d902b20e1bfa335c5d67f0d8c1e803e7b5f110b64e868dd0
[2025-02-23T20:09:24Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911313
[2025-02-23T20:09:24Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 1 (21911314 - 21911313)
[2025-02-23T20:09:37Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911315, hash: 0x66bd54e8de15637a55f4d963c81309ba1ece5101e6f9c4db19ad70c7b4f5be21
[2025-02-23T20:09:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911314
[2025-02-23T20:09:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 2 (21911315 - 21911313)
[2025-02-23T20:09:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] start: 21911313, end: 21911313
[2025-02-23T20:09:37Z DEBUG web3::transports::ws] [2] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5711",false],"id":2}
[2025-02-23T20:09:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911313
[2025-02-23T20:09:50Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911316, hash: 0xfb1f7b9fe7beb0050bc6a5ef1666e987549cc2a112f1ccf0a3ed2e1a4ab5dd40
[2025-02-23T20:09:50Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911315
[2025-02-23T20:09:50Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 3 (21911316 - 21911313)
[2025-02-23T20:09:50Z DEBUG uniswap_dai_usd_monitor::blocks_handler] start: 21911313, end: 21911314
[2025-02-23T20:09:50Z DEBUG web3::transports::ws] [3] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5711",false],"id":3}
[2025-02-23T20:09:50Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911313
[2025-02-23T20:09:50Z DEBUG web3::transports::ws] [4] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5712",false],"id":4}
[2025-02-23T20:09:50Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911314
[2025-02-23T20:10:00Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911317, hash: 0xf36a738c1f385f93f66629bbc862958c26dfa71a1ed96077640388fd2c227661
[2025-02-23T20:10:00Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911316
[2025-02-23T20:10:00Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 4 (21911317 - 21911313)
[2025-02-23T20:10:00Z DEBUG uniswap_dai_usd_monitor::blocks_handler] start: 21911313, end: 21911315
[2025-02-23T20:10:00Z DEBUG web3::transports::ws] [5] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5711",false],"id":5}
[2025-02-23T20:10:01Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911313
[2025-02-23T20:10:01Z DEBUG web3::transports::ws] [6] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5712",false],"id":6}
[2025-02-23T20:10:01Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911314
[2025-02-23T20:10:01Z DEBUG web3::transports::ws] [7] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5713",false],"id":7}
[2025-02-23T20:10:01Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911315
[2025-02-23T20:10:13Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911318, hash: 0x705fdf7d21d4f8ae7eef2df4145ccff451e7fd2a9f3867016bb14d40ad1d2f2a
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911317
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 5 (21911318 - 21911313)
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] start: 21911313, end: 21911316
[2025-02-23T20:10:13Z DEBUG web3::transports::ws] [8] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5711",false],"id":8}
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911313
[2025-02-23T20:10:13Z DEBUG web3::transports::ws] [9] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5712",false],"id":9}
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911314
[2025-02-23T20:10:13Z DEBUG web3::transports::ws] [10] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5713",false],"id":10}
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911315
[2025-02-23T20:10:13Z DEBUG web3::transports::ws] [11] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5714",false],"id":11}
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911316
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] ✅ N+5 condition met. Fetching events for block: 21911313 with hash: 0x4058daba514eb098522a9d0145a081503917291e641cd93fe2c6b5bddcfe0c6e
[2025-02-23T20:10:13Z DEBUG web3::transports::ws] [12] Calling: {"jsonrpc":"2.0","method":"eth_getLogs","params":[{"address":"0x5777d92f208679db4b9778590fa3cab3ac9e2168","blockHash":"0x4058daba514eb098522a9d0145a081503917291e641cd93fe2c6b5bddcfe0c6e","topics":["0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"]}],"id":12}
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] events not found
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] ⛔ block: 21911313 untracked
[2025-02-23T20:10:13Z DEBUG uniswap_dai_usd_monitor::blocks_handler] -----------------------
[2025-02-23T20:10:26Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911319, hash: 0x220fbfadc031df978ba5250a03e0d5622ebdf89c7eb656444853ada3a7f861a7
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911318
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 5 (21911319 - 21911314)
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] start: 21911314, end: 21911317
[2025-02-23T20:10:26Z DEBUG web3::transports::ws] [13] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5712",false],"id":13}
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911314
[2025-02-23T20:10:26Z DEBUG web3::transports::ws] [14] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5713",false],"id":14}
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911315
[2025-02-23T20:10:26Z DEBUG web3::transports::ws] [15] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5714",false],"id":15}
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911316
[2025-02-23T20:10:26Z DEBUG web3::transports::ws] [16] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5715",false],"id":16}
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911317
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] ✅ N+5 condition met. Fetching events for block: 21911314 with hash: 0x17bbd935e972b510d902b20e1bfa335c5d67f0d8c1e803e7b5f110b64e868dd0
[2025-02-23T20:10:26Z DEBUG web3::transports::ws] [17] Calling: {"jsonrpc":"2.0","method":"eth_getLogs","params":[{"address":"0x5777d92f208679db4b9778590fa3cab3ac9e2168","blockHash":"0x17bbd935e972b510d902b20e1bfa335c5d67f0d8c1e803e7b5f110b64e868dd0","topics":["0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"]}],"id":17}
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] events not found
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] ⛔ block: 21911314 untracked
[2025-02-23T20:10:26Z DEBUG uniswap_dai_usd_monitor::blocks_handler] -----------------------
[2025-02-23T20:10:36Z INFO  uniswap_dai_usd_monitor::blocks_handler] current block: 21911320, hash: 0x1027d6f26681a1a2de6c66cedb2253f1da09cf1d706638d54b2e5de4bccd0b76
[2025-02-23T20:10:36Z DEBUG uniswap_dai_usd_monitor::blocks_handler] √ parent hash matched for previous block: 21911319
[2025-02-23T20:10:36Z DEBUG uniswap_dai_usd_monitor::blocks_handler] diff = 5 (21911320 - 21911315)
[2025-02-23T20:10:36Z DEBUG uniswap_dai_usd_monitor::blocks_handler] start: 21911315, end: 21911318
[2025-02-23T20:10:36Z DEBUG web3::transports::ws] [18] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5713",false],"id":18}
[2025-02-23T20:10:36Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911315
[2025-02-23T20:10:36Z DEBUG web3::transports::ws] [19] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5714",false],"id":19}
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911316
[2025-02-23T20:10:37Z DEBUG web3::transports::ws] [20] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5715",false],"id":20}
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911317
[2025-02-23T20:10:37Z DEBUG web3::transports::ws] [21] Calling: {"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x14e5716",false],"id":21}
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] new hash matched for block: 21911318
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] ✅ N+5 condition met. Fetching events for block: 21911315 with hash: 0x66bd54e8de15637a55f4d963c81309ba1ece5101e6f9c4db19ad70c7b4f5be21
[2025-02-23T20:10:37Z DEBUG web3::transports::ws] [22] Calling: {"jsonrpc":"2.0","method":"eth_getLogs","params":[{"address":"0x5777d92f208679db4b9778590fa3cab3ac9e2168","blockHash":"0x66bd54e8de15637a55f4d963c81309ba1ece5101e6f9c4db19ad70c7b4f5be21","topics":["0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"]}],"id":22}
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] events not found
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] ⛔ block: 21911315 untracked
[2025-02-23T20:10:37Z DEBUG uniswap_dai_usd_monitor::blocks_handler] -----------------------

```

