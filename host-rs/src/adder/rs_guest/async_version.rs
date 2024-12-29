use crate::utils::get_component_linker_store;
use futures::executor::block_on;
use wasmtime::component::bindgen;
use wasmtime::{Engine, Result};

bindgen!({
    path: "../wit-files/adder.wit",
    world: "adder",
    async: true
});

pub fn run_adder_async(engine: &Engine) -> Result<()> {
    let (component, linker, mut store) = get_component_linker_store(
        engine,
        "./target/wasm32-wasip2/release/guest_adder_rs.wasm",
        "../target/wasm32-wasip2/release/guest_adder_rs.wasm",
    )?;
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
