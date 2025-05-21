use crate::utils::get_component_linker_store;
use wasmtime::component::bindgen;
use wasmtime::{Engine, Result};

bindgen!({
    path: "../wit-files/interfaced-adder.wit",
    world: "adder",
});

pub fn run_adder_sync(engine: &Engine) -> Result<()> {
    let (component, linker, mut store) = get_component_linker_store(
        engine,
        "./target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
        "../target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
    )?;
    let bindings = Adder::instantiate(&mut store, &component, &linker)?;
    let a = 1;
    let b = 2;
    // https://github.com/bytecodealliance/wasmtime/issues/9774#issuecomment-2539957106
    let interface_add = bindings.wasi_mindmap_interfaced_adder_add();
    let result = interface_add.call_add(&mut store, a, b)?;
    assert_eq!(result, 3);
    Ok(())
}

pub fn run_adder_dynamic(engine: &Engine) -> Result<()> {
    let (component, linker, mut store) = get_component_linker_store(
        engine,
        "./target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
        "../target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
    )?;
    let instance = linker.instantiate(&mut store, &component)?;
    let interface_name = "wasi-mindmap:interfaced-adder/add";
    let interface_idx = instance
        .get_export(&mut store, None, interface_name)
        .unwrap();
    let parent_export_idx = Some(&interface_idx.1);
    let func_name = "add";
    let func_idx = instance
        .get_export(&mut store, parent_export_idx, func_name)
        .unwrap();
    let func = instance.get_func(&mut store, func_idx.1).unwrap();
    // Reference:
    // * https://github.com/WebAssembly/wasi-cli/blob/main/wit/run.wit
    // * Documentation for [Func::typed](https://docs.rs/wasmtime/latest/wasmtime/component/struct.Func.html#method.typed) and [ComponentNamedList](https://docs.rs/wasmtime/latest/wasmtime/component/trait.ComponentNamedList.html)

    // If you don't know the types of arguments and return values of the function at compile time
    // iterate over the types of arguments at run time
    for (i, p) in func.params(&store).iter().enumerate() {
        println!("Type of {}th param: {:?}", i, p);
    }
    // iterate over the types of return values at run time
    for (i, r) in func.results(&store).iter().enumerate() {
        println!("Type of {}th result: {:?}", i, r);
    }

    // If you know the types of arguments and return values of the function at compile time
    let typed_func = func.typed::<(i32, i32), (i32,)>(&store)?;
    let (result,) = typed_func.call(&mut store, (1, 2))?;
    // Required, see documentation of TypedFunc::call
    typed_func.post_return(&mut store)?;
    assert_eq!(result, 3);
    Ok(())
}
