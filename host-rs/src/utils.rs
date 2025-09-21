use anyhow::Context;
use wasmtime::component::HasData;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::cli::{WasiCli, WasiCliView};
use wasmtime_wasi::filesystem::{WasiFilesystem, WasiFilesystemView};
use wasmtime_wasi::p2::bindings;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
// reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html
// reference: https://docs.wasmtime.dev/examples-rust-wasi.html
// reference: https://docs.rs/wasmtime/latest/wasmtime/component/trait.HasData.html

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

/// Copied from [`wasmtime_wasi::p2::HasIo`]
struct HasIo;

impl HasData for HasIo {
    type Data<'a> = &'a mut ResourceTable;
}

fn add_sync_io_to_linker<T: WasiView>(l: &mut Linker<T>) {
    wasmtime_wasi_io::bindings::wasi::io::error::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)
        .unwrap();
    bindings::sync::io::streams::add_to_linker::<T, HasIo>(l, |t| t.ctx().table).unwrap();
}

fn add_async_io_to_linker<T: WasiView>(l: &mut Linker<T>) {
    wasmtime_wasi_io::bindings::wasi::io::error::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)
        .unwrap();
    wasmtime_wasi_io::bindings::wasi::io::streams::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)
        .unwrap();
}

fn add_nonblocking_to_linker<'a, T: WasiView, O>(l: &mut Linker<T>, options: &'a O)
where
    bindings::cli::exit::LinkOptions: From<&'a O>,
{
    bindings::filesystem::preopens::add_to_linker::<T, WasiFilesystem>(l, T::filesystem).unwrap();
    bindings::cli::exit::add_to_linker::<T, WasiCli>(l, &options.into(), T::cli).unwrap();
    bindings::cli::environment::add_to_linker::<T, WasiCli>(l, T::cli).unwrap();
    bindings::cli::stdin::add_to_linker::<T, WasiCli>(l, T::cli).unwrap();
    bindings::cli::stdout::add_to_linker::<T, WasiCli>(l, T::cli).unwrap();
    bindings::cli::stderr::add_to_linker::<T, WasiCli>(l, T::cli).unwrap();
}

///
/// Bind WASI interfaces necessary for rust std in guest to run.
///
/// A pruned version of [`wasmtime_wasi::p2::add_to_linker_sync`],
/// [`wasmtime_wasi::p2::add_to_linker_async`], [`wasmtime_wasi::p2::add_to_linker_with_options_sync`] and [`wasmtime_wasi::p2::add_to_linker_with_options_async`]
///
/// For normal users, you can just use one of the above functions directly.
pub fn bind_interfaces_needed_by_guest_rust_std<T: WasiView>(l: &mut Linker<T>, r#async: bool) {
    if r#async {
        let options = bindings::LinkOptions::default();
        add_async_io_to_linker(l);
        add_nonblocking_to_linker(l, &options);
        bindings::filesystem::types::add_to_linker::<T, WasiFilesystem>(l, T::filesystem).unwrap();
    } else {
        let options = bindings::sync::LinkOptions::default();
        add_sync_io_to_linker(l);
        add_nonblocking_to_linker(l, &options);
        bindings::sync::filesystem::types::add_to_linker::<T, WasiFilesystem>(l, T::filesystem)
            .unwrap();
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
