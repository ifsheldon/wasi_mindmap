// Reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/index.html
mod async_version;
mod interfaced_sync_version;
mod sync_version;

pub use async_version::run_adder_async as run_adder_rs_async;
pub use interfaced_sync_version::run_adder_dynamic as run_interfaced_adder_dynamic;
pub use interfaced_sync_version::run_adder_sync as run_interfaced_adder_sync;
pub use sync_version::run_adder_sync as run_adder_rs_sync;

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::{Config, Engine, Result};
    #[test]
    fn test_adder_rs_sync() -> Result<()> {
        let engine = Engine::default();
        run_adder_rs_sync(&engine)
    }

    #[test]
    fn test_adder_rs_async() -> Result<()> {
        let mut config = Config::default();
        config.async_support(true);
        let engine_async = Engine::new(&config)?;
        run_adder_rs_async(&engine_async)
    }

    #[test]
    fn test_interfaced_adder_dynamic() -> Result<()> {
        let engine = Engine::default();
        run_interfaced_adder_dynamic(&engine)
    }

    #[test]
    fn test_interfaced_adder_sync() -> Result<()> {
        let engine = Engine::default();
        run_interfaced_adder_sync(&engine)
    }
}
