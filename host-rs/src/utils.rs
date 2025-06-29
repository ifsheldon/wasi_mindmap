use anyhow::Context;
use wasmtime::component::{Component, HasData, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::p2::{IoImpl, IoView, WasiImpl};
use wasmtime_wasi::p2::{WasiCtx, WasiCtxBuilder, WasiView};

// reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html
// reference: https://docs.wasmtime.dev/examples-rust-wasi.html
// reference: https://docs.rs/wasmtime/latest/wasmtime/component/trait.HasData.html

pub(crate) struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of WasiView and IoView
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl IoView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

impl WasiView for ComponentRunStates {
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

/// Copied from [`wasmtime_wasi::p2::HasIo`]
struct HasIo<T>(T);

impl<T: 'static> HasData for HasIo<T> {
    type Data<'a> = IoImpl<&'a mut T>;
}

/// Copied from [`wasmtime_wasi::p2::HasWasi`]
struct HasWasi<T>(T);

impl<T: 'static> HasData for HasWasi<T> {
    type Data<'a> = WasiImpl<&'a mut T>;
}

///
/// Bind WASI interfaces necessary for rust std in guest to run.
///
/// A pruned version of [`wasmtime_wasi::p2::add_to_linker_sync`] and [`wasmtime_wasi::p2::add_to_linker_with_options_sync`]
///
///
pub fn bind_interfaces_needed_by_guest_rust_std<T: WasiView + 'static>(l: &mut Linker<T>) {
    let f: fn(&mut T) -> IoImpl<&mut T> = |t| IoImpl(t);
    wasmtime_wasi::p2::bindings::io::error::add_to_linker::<T, HasIo<T>>(l, f).unwrap();
    wasmtime_wasi::p2::bindings::sync::io::streams::add_to_linker::<T, HasIo<T>>(l, f).unwrap();
    let f: fn(&mut T) -> WasiImpl<&mut T> = |t| WasiImpl(IoImpl(t));
    let options = wasmtime_wasi::p2::bindings::sync::LinkOptions::default();
    wasmtime_wasi::p2::bindings::sync::filesystem::types::add_to_linker::<T, HasWasi<T>>(l, f)
        .unwrap();
    wasmtime_wasi::p2::bindings::filesystem::preopens::add_to_linker::<T, HasWasi<T>>(l, f)
        .unwrap();
    wasmtime_wasi::p2::bindings::cli::exit::add_to_linker::<T, HasWasi<T>>(l, &options.into(), f)
        .unwrap();
    wasmtime_wasi::p2::bindings::cli::environment::add_to_linker::<T, HasWasi<T>>(l, f).unwrap();
    wasmtime_wasi::p2::bindings::cli::stdin::add_to_linker::<T, HasWasi<T>>(l, f).unwrap();
    wasmtime_wasi::p2::bindings::cli::stdout::add_to_linker::<T, HasWasi<T>>(l, f).unwrap();
    wasmtime_wasi::p2::bindings::cli::stderr::add_to_linker::<T, HasWasi<T>>(l, f).unwrap();
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
