mod async_version;
mod sync_version;

pub use async_version::run_kv_store_async as run_kv_store_rs_async;
pub use sync_version::run_kv_store_sync as run_kv_store_rs_sync;

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::{Config, Engine, Result};

    #[test]
    fn test_kv_store_rs_sync() -> Result<()> {
        let engine = Engine::default();
        run_kv_store_rs_sync(&engine)
    }

    #[test]
    fn test_kv_store_rs_async() -> Result<()> {
        let mut config = Config::default();
        config.async_support(true);
        let engine_async = Engine::new(&config)?;
        run_kv_store_rs_async(&engine_async)
    }
}
