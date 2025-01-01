mod async_version;
mod sync_version;

pub use async_version::run_adder_async as run_adder_py_async;
pub use sync_version::run_adder_sync as run_adder_py_sync;

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::{Config, Engine, Result};

    #[test]
    fn test_adder_py_async() -> Result<()> {
        let mut config = Config::default();
        config.async_support(true);
        let engine_async = Engine::new(&config)?;
        run_adder_py_async(&engine_async)
    }

    #[test]
    fn test_adder_py_sync() -> Result<()> {
        let engine = Engine::default();
        run_adder_py_sync(&engine)
    }
}
