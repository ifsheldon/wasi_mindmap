# WASI Mindmap

A collection of examples and tutorials about WASIp2.

Complementary
guide - [A Complete Guide to WASIp2 for Rust and Python Programmers](https://ideas.reify.ing/en/blog/complete-guide-to-wasip2-for-rust-python-programmers/)

## Table of examples

| Host/Guest                    | Rust Adder [↪](./guest-adder-rs)   | Python Adder [↪](./guest-adder-py) | Rust KVDatabase [↪](./guest-kv-store-rs) |
|-------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------------|
| Rust Host [↪](./host-rs)      | ✅[↪](./host-rs/src/adder/rs_guest) | ✅[↪](./host-rs/src/adder/py_guest) | ✅[↪](./host-rs/src/kv_store/rs_guest)    |
| Python Host [↪](./host-py)    | ✅                                  | 🛠️                                | 🛠️                                      |
| Command Component (from Rust) | ✅[↪](./host-command-component)     | 📌                                 | 📌                                       |

✅: Currently supported

🛠️: Not supported for now, under development of `wasmtime-py`

📌: TODO and welcome contributions

## Related Issues:

* Resolved ones by me, just FYI:
    * [Missing examples for using bindgen! async, imports and resource in host](https://github.com/bytecodealliance/wasmtime/issues/9776)
    * [Bindgen improvement: Remove the use of async_trait](https://github.com/bytecodealliance/wasmtime/issues/9823)
    * [Documentation: Wrong doc about Config::wasm_component_model](https://github.com/bytecodealliance/wasmtime/issues/9694)
    * [Renovate host example with latest wasmtime and wasmtime_wasi](https://github.com/bytecodealliance/component-docs/issues/179)
* Unresolved ones, for those who may be interested in contributing:
    * [Compiled wasm32-wasip2 component from simple code requires excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235)
    * [Renovate the WASI example](https://github.com/bytecodealliance/wasmtime/issues/9777)
    * [Bindgen! gives weird name to an interface well-named in WIT file](https://github.com/bytecodealliance/wasmtime/issues/9774)
* TODO: Add an improvement issue regarding `Func::post_return`, probably can
  use [Undroppable Types](https://jack.wrenn.fyi/blog/undroppable/)