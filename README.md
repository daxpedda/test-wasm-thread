cargo build && wasm-bindgen --no-modules --no-typescript --out-dir web target/wasm32-unknown-unknown/debug/test_wasm_thread.wasm
http --header "cross-origin-opener-policy: same-origin" --header "cross-origin-embedder-policy: require-corp" -- web
