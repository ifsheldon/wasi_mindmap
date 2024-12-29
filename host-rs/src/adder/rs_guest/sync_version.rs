use crate::utils::get_component_linker_store;
use wasmtime::component::bindgen;
use wasmtime::{Engine, Result};

bindgen!({
    path: "../wit-files/adder.wit",
    world: "adder",
});

pub fn run_adder_sync(engine: &Engine) -> Result<()> {
    let (component, linker, mut store) = get_component_linker_store(
        engine,
        "./target/wasm32-wasip2/release/guest_adder_rs.wasm",
        "../target/wasm32-wasip2/release/guest_adder_rs.wasm",
    )?;
    let bindings = Adder::instantiate(&mut store, &component, &linker)?;
    let a = 1;
    let b = 2;
    let result = bindings.call_add(&mut store, a, b)?;
    assert_eq!(result, 3);
    Ok(())
}
