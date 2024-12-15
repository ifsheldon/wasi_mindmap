use crate::utils::get_component_linker_store;
use wasmtime::component::bindgen;
use wasmtime::Engine;

pub use async_version::run_adder_async as run_adder_rs_async;
pub use interfaced_sync_version::run_adder_dynamic as run_interfaced_adder_dynamic;
pub use interfaced_sync_version::run_adder_sync as run_interfaced_adder_sync;
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

mod interfaced_sync_version {
    use super::*;

    bindgen!({
        path: "../wit-files/interfaced-adder.wit",
        world: "adder",
    });

    pub fn run_adder_sync(engine: &Engine) {
        let (component, linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
            "../target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
        );
        let bindings = Adder::instantiate(&mut store, &component, &linker).unwrap();
        let a = 1;
        let b = 2;
        // https://github.com/bytecodealliance/wasmtime/issues/9774#issuecomment-2539957106
        let interface_add = bindings.component_interfaced_adder_add();
        let result = interface_add.call_add(&mut store, a, b).unwrap();
        assert_eq!(result, 3);
    }

    pub fn run_adder_dynamic(engine: &Engine) {
        let (component, linker, mut store) = get_component_linker_store(
            engine,
            "./target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
            "../target/wasm32-wasip2/release/guest_interfaced_adder_rs.wasm",
        );
        let instance = linker.instantiate(&mut store, &component).unwrap();
        let interface_name = "component:interfaced-adder/add";
        let interface_idx = instance
            .get_export(&mut store, None, interface_name)
            .unwrap();
        let parent_export_idx = Some(&interface_idx);
        let func_name = "add";
        let func_idx = instance
            .get_export(&mut store, parent_export_idx, func_name)
            .unwrap();
        let func = instance.get_func(&mut store, func_idx).unwrap();
        // Reference:
        // * https://github.com/WebAssembly/wasi-cli/blob/main/wit/run.wit
        // * Documentation for [Func::typed](https://docs.rs/wasmtime/latest/wasmtime/component/struct.Func.html#method.typed) and [ComponentNamedList](https://docs.rs/wasmtime/latest/wasmtime/component/trait.ComponentNamedList.html)

        // If you don't know the types of arguments and return values of the function
        // iterate over the types of arguments
        for (i, p) in func.params(&store).iter().enumerate() {
            println!("Type of {}th param: {:?}", i, p);
        }
        // iterate over the types of return values
        for (i, r) in func.results(&store).iter().enumerate() {
            println!("Type of {}th result: {:?}", i, r);
        }

        // If you know the types of arguments and return values of the function
        let typed_func = func.typed::<(i32, i32), (i32,)>(&store).unwrap();
        let (result,) = typed_func.call(&mut store, (1, 2)).unwrap();
        // Required, see documentation of TypedFunc::call
        typed_func.post_return(&mut store).unwrap();
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
