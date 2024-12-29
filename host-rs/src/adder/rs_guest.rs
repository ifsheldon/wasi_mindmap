// Reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/index.html
mod async_version;
mod interfaced_sync_version;
mod sync_version;

pub use async_version::run_adder_async as run_adder_rs_async;
pub use interfaced_sync_version::run_adder_dynamic as run_interfaced_adder_dynamic;
pub use interfaced_sync_version::run_adder_sync as run_interfaced_adder_sync;
pub use sync_version::run_adder_sync as run_adder_rs_sync;
