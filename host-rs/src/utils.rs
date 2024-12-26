use anyhow::Context;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::WasiImpl;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

// reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html
// reference: https://docs.wasmtime.dev/examples-rust-wasi.html

pub(crate) struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of WasiView
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
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

/// Copied from [wasmtime_wasi::type_annotate]
pub fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}

///
/// Bind WASI interfaces necessary for rust std in guest to run.
///
/// A pruned version of [`wasmtime_wasi::add_to_linker_sync`] and [`wasmtime_wasi::add_to_linker_with_options_sync`]
///
///
pub fn bind_interfaces_needed_by_guest_rust_std<T: WasiView>(l: &mut Linker<T>) {
    let closure = type_annotate::<T, _>(|t| WasiImpl(t));
    let options = wasmtime_wasi::bindings::sync::LinkOptions::default();
    wasmtime_wasi::bindings::sync::filesystem::types::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::filesystem::preopens::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::io::error::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::sync::io::streams::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::cli::exit::add_to_linker_get_host(l, &options.into(), closure)
        .unwrap();
    wasmtime_wasi::bindings::cli::environment::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::cli::stdin::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::cli::stdout::add_to_linker_get_host(l, closure).unwrap();
    wasmtime_wasi::bindings::cli::stderr::add_to_linker_get_host(l, closure).unwrap();
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
        .or_else(|_| Component::from_file(&engine, alt_path))
        .with_context(|| format!("Cannot find component from path: {path} or {alt_path}"))?;
    let linker = Linker::new(&engine);
    let state = ComponentRunStates::new();
    let store = Store::new(&engine, state);
    Ok((component, linker, store))
}
