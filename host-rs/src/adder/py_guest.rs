mod async_version;
mod sync_version;

pub use async_version::run_adder_async as run_adder_py_async;
pub use sync_version::run_adder_sync as run_adder_py_sync;
