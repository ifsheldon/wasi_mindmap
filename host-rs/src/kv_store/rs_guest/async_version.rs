// Reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_4_imported_resources/index.html

// use async_trait::async_trait; // before wasmtime 29.0, this is needed
use futures::executor::block_on;

use crate::utils::get_component_linker_store;
use crate::utils::{ComponentRunStates, bind_interfaces_needed_by_guest_rust_std};
use std::collections::HashMap;
use wasmtime::component::bindgen;
use wasmtime::component::{HasSelf, Resource};
use wasmtime::{Engine, Result};

bindgen!({
    path: "../wit-files/kv-store.wit",
    world: "kv-database",
    // Interactions with `ResourceTable` can possibly trap so enable the ability
    // to return traps from generated functions.
    imports: { default: async | trappable },
    exports: { default: async },
    with: {
        "wasi-mindmap:kv-store/kvdb/connection": Connection
    },
});

pub struct Connection {
    pub storage: HashMap<String, String>,
}

// #[async_trait] // before wasmtime 29.0, this is needed
impl KvDatabaseImports for ComponentRunStates {
    async fn log(&mut self, msg: String) -> Result<(), wasmtime::Error> {
        println!("Log: {msg}");
        Ok(())
    }
}

impl wasi_mindmap::kv_store::kvdb::Host for ComponentRunStates {}

// #[async_trait] // before wasmtime 29.0, this is needed
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
        Ok(connection.storage.get(&key).cloned())
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
    KvDatabase::add_to_linker::<_, HasSelf<_>>(&mut linker, |s| s)?;
    bind_interfaces_needed_by_guest_rust_std(&mut linker, true);
    let async_future = async {
        let bindings = KvDatabase::instantiate_async(&mut store, &component, &linker).await?;
        let result = bindings.call_replace_value(store, "hello", "world").await?;
        assert_eq!(result, None);
        Ok(())
    };
    block_on(async_future)
}
