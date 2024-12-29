use crate::utils::get_component_linker_store;
use futures::executor::block_on;
use wasmtime::component::bindgen;
use wasmtime::{Engine, Result};
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
