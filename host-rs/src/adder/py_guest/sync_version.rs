use crate::utils::get_component_linker_store;
use wasmtime::component::bindgen;
use wasmtime::{Engine, Result};
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
