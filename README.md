# wasm_websocket_rpc_client
RPC over WebSocket with MessagePack + WebAssembly + Rust client

## How to use

```shell
# make sure wry_websocket_rpc_server is running in separate tab
wasm-pack build --dev --target web && npx http-server -c-1
# visit http://localhost:8080/assets/index.html and check console logs
```
