use crate::utils::get_component_linker_store;
use crate::utils::{bind_interfaces_needed_by_guest_rust_std, ComponentRunStates};
use std::collections::HashMap;
use wasmtime::component::bindgen;
use wasmtime::component::Resource;
use wasmtime::{Engine, Result};

pub use async_version::run_kv_store_async as run_kv_store_rs_async;
pub use sync_version::run_kv_store_sync as run_kv_store_rs_sync;

// Reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_4_imported_resources/index.html

mod sync_version {
    use super::*;

    bindgen!({
        path: "../wit-files/kv-store.wit",
        world: "kv-database",
        with: {
            "wasi-mindmap:kv-store/kvdb/connection": Connection
        },
        // Interactions with `ResourceTable` can possibly trap so enable the ability
        // to return traps from generated functions.
        trappable_imports: true,
    });

    pub struct Connection {
        pub storage: HashMap<String, String>,
    }

    impl KvDatabaseImports for ComponentRunStates {
        fn log(&mut self, msg: String) -> Result<(), wasmtime::Error> {
            println!("Log: {}", msg);
            Ok(())
        }
    }

    impl wasi_mindmap::kv_store::kvdb::Host for ComponentRunStates {}

    impl wasi_mindmap::kv_store::kvdb::HostConnection for ComponentRunStates {
        fn new(&mut self) -> Result<Resource<Connection>, wasmtime::Error> {
            Ok(self.resource_table.push(Connection {
                storage: HashMap::new(),
            })?)
        }

        fn get(
            &mut self,
            resource: Resource<Connection>,
            key: String,
        ) -> Result<Option<String>, wasmtime::Error> {
            let connection = self.resource_table.get(&resource)?;
            Ok(connection.storage.get(&key).map(String::clone))
        }

        fn set(
            &mut self,
            resource: Resource<Connection>,
            key: String,
            value: String,
        ) -> Result<()> {
            let connection = self.resource_table.get_mut(&resource)?;
            connection.storage.insert(key, value);
            Ok(())
        }

        fn remove(
            &mut self,
            resource: Resource<Connection>,
            key: String,
        ) -> Result<Option<String>> {
            let connection = self.resource_table.get_mut(&resource)?;
            Ok(connection.storage.remove(&key))
        }

        fn clear(&mut self, resource: Resource<Connection>) -> Result<(), wasmtime::Error> {
            let large_string = self.resource_table.get_mut(&resource)?;
            large_string.storage.clear();
            Ok(())
        }

        fn drop(&mut self, resource: Resource<Connection>) -> Result<()> {
            let _ = self.resource_table.delete(resource)?;
            Ok(())
        }
    }

    pub fn run_kv_store_sync(engine: &Engine) -> Result<()> {
        let (component, mut linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_kv_store_rs.wasm",
            "../target/wasm32-wasip2/release/guest_kv_store_rs.wasm",
        )?;
        KvDatabase::add_to_linker(&mut linker, |s| s)?;
        bind_interfaces_needed_by_guest_rust_std(&mut linker);
        let bindings = KvDatabase::instantiate(&mut store, &component, &linker)?;
        let result = bindings.call_replace_value(store, "hello", "world")?;
        assert_eq!(result, None);
        Ok(())
    }
}

mod async_version {
    use async_trait::async_trait;
    use futures::executor::block_on;

    use super::*;

    bindgen!({
        path: "../wit-files/kv-store.wit",
        world: "kv-database",
        async: true,
        with: {
            "wasi-mindmap:kv-store/kvdb/connection": Connection
        },
        // Interactions with `ResourceTable` can possibly trap so enable the ability
        // to return traps from generated functions.
        trappable_imports: true,
    });

    pub struct Connection {
        pub storage: HashMap<String, String>,
    }

    #[async_trait]
    impl KvDatabaseImports for ComponentRunStates {
        async fn log(&mut self, msg: String) -> Result<(), wasmtime::Error> {
            println!("Log: {}", msg);
            Ok(())
        }
    }

    impl wasi_mindmap::kv_store::kvdb::Host for ComponentRunStates {}

    #[async_trait]
    impl wasi_mindmap::kv_store::kvdb::HostConnection for ComponentRunStates {
        async fn new(&mut self) -> Result<Resource<Connection>, wasmtime::Error> {
            Ok(self.resource_table.push(Connection {
                storage: HashMap::new(),
            })?)
        }

        async fn get(
            &mut self,
            resource: Resource<Connection>,
            key: String,
        ) -> Result<Option<String>, wasmtime::Error> {
            let connection = self.resource_table.get(&resource)?;
            Ok(connection.storage.get(&key).map(String::clone))
        }

        async fn set(
            &mut self,
            resource: Resource<Connection>,
            key: String,
            value: String,
        ) -> Result<()> {
            let connection = self.resource_table.get_mut(&resource)?;
            connection.storage.insert(key, value);
            Ok(())
        }

        async fn remove(
            &mut self,
            resource: Resource<Connection>,
            key: String,
        ) -> Result<Option<String>> {
            let connection = self.resource_table.get_mut(&resource)?;
            Ok(connection.storage.remove(&key))
        }

        async fn clear(&mut self, resource: Resource<Connection>) -> Result<(), wasmtime::Error> {
            let large_string = self.resource_table.get_mut(&resource)?;
            large_string.storage.clear();
            Ok(())
        }

        async fn drop(&mut self, resource: Resource<Connection>) -> Result<()> {
            let _ = self.resource_table.delete(resource)?;
            Ok(())
        }
    }

    pub fn run_kv_store_async(engine: &Engine) -> Result<()> {
        let (component, mut linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_kv_store_rs.wasm",
            "../target/wasm32-wasip2/release/guest_kv_store_rs.wasm",
        )?;
        KvDatabase::add_to_linker(&mut linker, |s| s)?;
        bind_interfaces_needed_by_guest_rust_std(&mut linker);
        let async_future = async {
            let bindings = KvDatabase::instantiate_async(&mut store, &component, &linker).await?;
            let result = bindings.call_replace_value(store, "hello", "world").await?;
            assert_eq!(result, None);
            Ok(())
        };
        block_on(async_future)
    }
}
