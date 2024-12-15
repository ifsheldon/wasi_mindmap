use crate::utils::get_component_linker_store;
use wasmtime::component::bindgen;
use wasmtime::{Engine, Result};

pub use async_version::run_adder_async as run_adder_py_async;
pub use sync_version::run_adder_sync as run_adder_py_sync;
// https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/index.html
mod sync_version {
    use super::*;
    use wasmtime_wasi::add_to_linker_sync;

    bindgen!({
        path: "../wit-files/adder.wit",
        world: "adder",
    });

    pub fn run_adder_sync(engine: &Engine) -> Result<()> {
        println!("Loading guest-adder-py, will take dozens of seconds");
        let (component, mut linker, mut store) = get_component_linker_store(
            engine,
            "./guest-adder-py/guest_adder_py.wasm",
            "../guest-adder-py/guest_adder_py.wasm",
        )?;
        add_to_linker_sync(&mut linker)?;
        let bindings = Adder::instantiate(&mut store, &component, &linker)?;
        let a = 1;
        let b = 2;
        let result = bindings.call_add(&mut store, a, b)?;
        assert_eq!(result, 3);
        Ok(())
    }
}

mod async_version {
    use super::*;
    use futures::executor::block_on;
    use wasmtime_wasi::add_to_linker_async;

    bindgen!({
        path: "../wit-files/adder.wit",
        world: "adder",
        async: true
    });

    pub fn run_adder_async(engine: &Engine) -> Result<()> {
        println!("Loading guest-adder-py, will take dozens of seconds");
        let (component, mut linker, mut store) = get_component_linker_store(
            engine,
            "./guest-adder-py/guest_adder_py.wasm",
            "../guest-adder-py/guest_adder_py.wasm",
        )?;
        add_to_linker_async(&mut linker)?;
        let async_future = async {
            let bindings = Adder::instantiate_async(&mut store, &component, &linker).await?;
            let a = 1;
            let b = 2;
            let result = bindings.call_add(&mut store, a, b).await?;
            assert_eq!(result, 3);
            Ok(())
        };
        block_on(async_future)
    }
}
