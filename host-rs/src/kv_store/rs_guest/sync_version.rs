// Reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_4_imported_resources/index.html

use crate::utils::get_component_linker_store;
use crate::utils::{ComponentRunStates, bind_interfaces_needed_by_guest_rust_std};
use std::collections::HashMap;
use wasmtime::component::bindgen;
use wasmtime::component::{HasSelf, Resource};
use wasmtime::{Engine, Result};

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

    fn set(&mut self, resource: Resource<Connection>, key: String, value: String) -> Result<()> {
        let connection = self.resource_table.get_mut(&resource)?;
        connection.storage.insert(key, value);
        Ok(())
    }

    fn remove(&mut self, resource: Resource<Connection>, key: String) -> Result<Option<String>> {
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
    KvDatabase::add_to_linker::<_, HasSelf<_>>(&mut linker, |s| s)?;
    bind_interfaces_needed_by_guest_rust_std(&mut linker);
    let bindings = KvDatabase::instantiate(&mut store, &component, &linker)?;
    let result = bindings.call_replace_value(store, "hello", "world")?;
    assert_eq!(result, None);
    Ok(())
}
