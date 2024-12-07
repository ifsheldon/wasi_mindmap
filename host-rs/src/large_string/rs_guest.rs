use crate::utils::get_component_linker_store;
use crate::utils::{bind_interfaces_needed_by_guest_rust_std, ComponentRunStates};
use wasmtime::component::bindgen;
use wasmtime::component::Resource;
use wasmtime::Engine;

pub use async_version::run_large_string_async as run_large_string_rs_async;
pub use sync_version::run_large_string_sync as run_large_string_rs_sync;

mod sync_version {
    use super::*;

    bindgen!({
        path: "../wit-files/string.wit",
        world: "big-string",
        with: {
            "component:big-string/large-string/largestring": LargeString
        }
    });

    pub struct LargeString {
        pub storage: String,
    }

    impl BigStringImports for ComponentRunStates {
        fn print(&mut self, string: String) -> bool {
            println!("from print sync host func: {}", string);
            true
        }
    }

    impl component::big_string::large_string::Host for ComponentRunStates {}

    impl component::big_string::large_string::HostLargestring for ComponentRunStates {
        fn new(&mut self) -> Resource<LargeString> {
            self.resource_table
                .push(LargeString {
                    storage: String::new(),
                })
                .unwrap()
        }

        fn push(&mut self, resource: Resource<LargeString>, s: String) -> () {
            let large_string = self.resource_table.get_mut(&resource).unwrap();
            large_string.storage.push_str(s.as_str());
        }

        fn get(&mut self, resource: Resource<LargeString>) -> String {
            let large_string = self.resource_table.get(&resource).unwrap();
            format!("sync: {}", large_string.storage)
        }

        fn clear(&mut self, resource: Resource<LargeString>) -> () {
            let large_string = self.resource_table.get_mut(&resource).unwrap();
            large_string.storage.clear();
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
    use futures::executor::block_on;

    use super::*;
    use core::future::Future;
    use core::pin::Pin;

    bindgen!({
        path: "../wit-files/string.wit",
        world: "big-string",
        async: true,
        with: {
            "component:big-string/large-string/largestring": LargeString
        },
    });

    pub struct LargeString {
        pub storage: String,
    }

    impl BigStringImports for ComponentRunStates {
        fn print<'life, 'async_trait>(
            &'life mut self,
            s: String,
        ) -> Pin<Box<dyn Future<Output = bool> + Send + 'async_trait>>
        where
            'life: 'async_trait,
            Self: 'async_trait,
        {
            // TODO: make a PR for this
            Box::pin(async move {
                println!("from print async host func: {}", s);
                true
            })
        }
    }

    impl component::big_string::large_string::Host for ComponentRunStates {}

    impl component::big_string::large_string::HostLargestring for ComponentRunStates {
        fn new<'life, 'async_trait>(
            &'life mut self,
        ) -> Pin<Box<dyn Future<Output = Resource<LargeString>> + Send + 'async_trait>>
        where
            'life: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async {
                self.resource_table
                    .push(LargeString {
                        storage: String::new(),
                    })
                    .unwrap()
            })
        }

        fn push<'life, 'async_trait>(
            &'life mut self,
            resource: Resource<LargeString>,
            s: String,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
        where
            'life: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let large_string = self.resource_table.get_mut(&resource).unwrap();
                large_string.storage.push_str(s.as_str());
            })
        }

        fn get<'life, 'async_trait>(
            &'life mut self,
            resource: Resource<LargeString>,
        ) -> Pin<Box<dyn Future<Output = String> + Send + 'async_trait>>
        where
            'life: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let large_string = self.resource_table.get(&resource).unwrap();
                format!("async: {}", large_string.storage)
            })
        }

        fn clear<'life, 'async_trait>(
            &'life mut self,
            resource: Resource<LargeString>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
        where
            'life: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let large_string = self.resource_table.get_mut(&resource).unwrap();
                large_string.storage.clear();
            })
        }

        fn drop<'life, 'async_trait>(
            &'life mut self,
            resource: Resource<LargeString>,
        ) -> Pin<Box<dyn Future<Output = wasmtime::Result<()>> + Send + 'async_trait>>
        where
            'life: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async {
                let _ = self.resource_table.delete(resource)?;
                Ok(())
            })
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
