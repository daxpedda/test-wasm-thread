### Build
`cargo build && wasm-bindgen --no-modules --no-typescript --out-dir web target/wasm32-unknown-unknown/debug/test_wasm_thread.wasm`

### Host
See https://crates.io/crates/https or use any other HTTP hosting solution.
`http --header "cross-origin-opener-policy: same-origin" --header "cross-origin-embedder-policy: require-corp" -- web`
