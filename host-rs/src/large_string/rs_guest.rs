use crate::utils::get_component_linker_store;
use crate::utils::{bind_interfaces_needed_by_guest_rust_std, ComponentRunStates};
use wasmtime::component::bindgen;
use wasmtime::component::Resource;
use wasmtime::Engine;

pub use async_version::run_large_string_async as run_large_string_rs_async;
pub use sync_version::run_large_string_sync as run_large_string_rs_sync;

// Reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_4_imported_resources/index.html

mod sync_version {
    use super::*;

    bindgen!({
        path: "../wit-files/string.wit",
        world: "big-string",
        with: {
            "component:big-string/large-string/largestring": LargeString
        },
        // Interactions with `ResourceTable` can possibly trap so enable the ability
        // to return traps from generated functions.
        trappable_imports: true,
    });

    pub struct LargeString {
        pub storage: String,
    }

    impl BigStringImports for ComponentRunStates {
        fn print(&mut self, string: String) -> Result<bool, wasmtime::Error> {
            println!("from print sync host func: {}", string);
            Ok(true)
        }
    }

    impl component::big_string::large_string::Host for ComponentRunStates {}

    impl component::big_string::large_string::HostLargestring for ComponentRunStates {
        fn new(&mut self) -> Result<Resource<LargeString>, wasmtime::Error> {
            Ok(self.resource_table.push(LargeString {
                storage: String::new(),
            })?)
        }

        fn push(
            &mut self,
            resource: Resource<LargeString>,
            s: String,
        ) -> Result<(), wasmtime::Error> {
            let large_string = self.resource_table.get_mut(&resource).unwrap();
            large_string.storage.push_str(s.as_str());
            Ok(())
        }

        fn get(&mut self, resource: Resource<LargeString>) -> Result<String, wasmtime::Error> {
            let large_string = self.resource_table.get(&resource)?;
            Ok(format!("sync: {}", large_string.storage))
        }

        fn clear(&mut self, resource: Resource<LargeString>) -> Result<(), wasmtime::Error> {
            let large_string = self.resource_table.get_mut(&resource)?;
            large_string.storage.clear();
            Ok(())
        }

        fn drop(&mut self, resource: Resource<LargeString>) -> wasmtime::Result<()> {
            let _ = self.resource_table.delete(resource)?;
            Ok(())
        }
    }

    pub fn run_large_string_sync(engine: &Engine) {
        let (component, mut linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_largestring_rs.wasm",
            "../target/wasm32-wasip2/release/guest_largestring_rs.wasm",
        );
        BigString::add_to_linker(&mut linker, |s| s).unwrap();
        bind_interfaces_needed_by_guest_rust_std(&mut linker);
        let bindings = BigString::instantiate(&mut store, &component, &linker).unwrap();
        let result = bindings.call_manipulate_large_string(store).unwrap();
        assert_eq!(result, "sync: ab");
    }
}

mod async_version {
    use async_trait::async_trait;
    use futures::executor::block_on;

    use super::*;

    bindgen!({
        path: "../wit-files/string.wit",
        world: "big-string",
        async: true,
        with: {
            "component:big-string/large-string/largestring": LargeString
        },
        // Interactions with `ResourceTable` can possibly trap so enable the ability
        // to return traps from generated functions.
        trappable_imports: true,
    });

    pub struct LargeString {
        pub storage: String,
    }

    #[async_trait]
    impl BigStringImports for ComponentRunStates {
        async fn print(&mut self, s: String) -> Result<bool, wasmtime::Error> {
            println!("from print async host func: {}", s);
            Ok(true)
        }
    }

    impl component::big_string::large_string::Host for ComponentRunStates {}

    #[async_trait]
    impl component::big_string::large_string::HostLargestring for ComponentRunStates {
        async fn new(&mut self) -> Result<Resource<LargeString>, wasmtime::Error> {
            Ok(self.resource_table.push(LargeString {
                storage: String::new(),
            })?)
        }

        async fn push(
            &mut self,
            resource: Resource<LargeString>,
            s: String,
        ) -> Result<(), wasmtime::Error> {
            let large_string = self.resource_table.get_mut(&resource)?;
            large_string.storage.push_str(s.as_str());
            Ok(())
        }

        async fn get(
            &mut self,
            resource: Resource<LargeString>,
        ) -> Result<String, wasmtime::Error> {
            let large_string = self.resource_table.get(&resource)?;
            Ok(format!("async: {}", large_string.storage))
        }

        async fn clear(&mut self, resource: Resource<LargeString>) -> Result<(), wasmtime::Error> {
            let large_string = self.resource_table.get_mut(&resource)?;
            large_string.storage.clear();
            Ok(())
        }

        async fn drop(&mut self, resource: Resource<LargeString>) -> wasmtime::Result<()> {
            let _ = self.resource_table.delete(resource)?;
            Ok(())
        }
    }

    pub fn run_large_string_async(engine: &Engine) {
        let (component, mut linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_largestring_rs.wasm",
            "../target/wasm32-wasip2/release/guest_largestring_rs.wasm",
        );
        BigString::add_to_linker(&mut linker, |s| s).unwrap();
        bind_interfaces_needed_by_guest_rust_std(&mut linker);
        let async_future = async {
            let bindings = BigString::instantiate_async(&mut store, &component, &linker)
                .await
                .unwrap();
            let result = bindings.call_manipulate_large_string(store).await.unwrap();
            assert_eq!(result, "async: ab");
        };
        block_on(async_future);
    }
}
