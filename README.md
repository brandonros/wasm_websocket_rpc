# wasm_websocket_rpc
RPC over WebSocket with MessagePack + WebAssembly + Rust

## How to use

```shell
cargo install wasm-pack
# make sure project is [lib] crate-type = ["cdylib"] and not bin
wasm-pack build --dev --target web && npx http-server -c-1
# visit http://localhost:8080
```
