use wasmtime::error::Context;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
// reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html
// reference: https://docs.wasmtime.dev/examples-rust-wasi.html

pub(crate) struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of WasiView
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl ComponentRunStates {
    pub fn new() -> Self {
        ComponentRunStates {
            wasi_ctx: WasiCtxBuilder::new().build(),
            resource_table: ResourceTable::new(),
        }
    }
}

///
/// Bind WASI interfaces necessary for rust std in guest to run.
///
/// You can use other variants, like [`wasmtime_wasi::p2::add_to_linker_with_options_sync`] and [`wasmtime_wasi::p2::add_to_linker_with_options_async`]
///
pub fn bind_interfaces_needed_by_guest_rust_std<T: WasiView>(l: &mut Linker<T>, r#async: bool) {
    if r#async {
        wasmtime_wasi::p2::add_to_linker_async(l).unwrap();
    } else {
        wasmtime_wasi::p2::add_to_linker_sync(l).unwrap();
    }
}

pub fn get_component_linker_store(
    engine: &Engine,
    path: &'static str,
    alt_path: &'static str,
) -> Result<(
    Component,
    Linker<ComponentRunStates>,
    Store<ComponentRunStates>,
)> {
    let component = Component::from_file(engine, path)
        .or_else(|_| Component::from_file(engine, alt_path))
        .with_context(|| format!("Cannot find component from path: {path} or {alt_path}"))?;
    let linker = Linker::new(engine);
    let state = ComponentRunStates::new();
    let store = Store::new(engine, state);
    Ok((component, linker, store))
}
