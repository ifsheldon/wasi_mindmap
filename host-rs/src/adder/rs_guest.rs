use crate::utils::get_component_linker_store;
use wasmtime::component::bindgen;
use wasmtime::Engine;

pub use async_version::run_adder_async as run_adder_rs_async;
pub use sync_version::run_adder_sync as run_adder_rs_sync;
// https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/index.html
mod sync_version {
    use super::*;

    bindgen!({
        path: "../wit-files/adder.wit",
        world: "adder",
    });

    pub fn run_adder_sync(engine: &Engine) {
        let (component, linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_adder_rs.wasm",
            "../target/wasm32-wasip2/release/guest_adder_rs.wasm",
        );
        let bindings = Adder::instantiate(&mut store, &component, &linker).unwrap();
        let a = 1;
        let b = 2;
        let result = bindings.call_add(&mut store, a, b).unwrap();
        assert_eq!(result, 3);
    }
}

mod async_version {
    use super::*;
    use futures::executor::block_on;

    bindgen!({
        path: "../wit-files/adder.wit",
        world: "adder",
        async: true
    });

    pub fn run_adder_async(engine: &Engine) {
        let (component, linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_adder_rs.wasm",
            "../target/wasm32-wasip2/release/guest_adder_rs.wasm",
        );
        let async_future = async {
            let bindings = Adder::instantiate_async(&mut store, &component, &linker)
                .await
                .unwrap();
            let a = 1;
            let b = 2;
            let result = bindings.call_add(&mut store, a, b).await.unwrap();
            assert_eq!(result, 3);
        };
        block_on(async_future);
    }
}
