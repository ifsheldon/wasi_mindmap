use crate::adder::*;
use crate::kv_store::*;
use clap::{Parser, ValueEnum};
use wasmtime::Result;
use wasmtime::Engine;

mod adder;
mod kv_store;
mod utils;

// reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html
// reference: https://docs.wasmtime.dev/examples-rust-wasi.html

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    module: Option<Module>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Module {
    All,
    RustAdder,
    RustKVStore,
    PythonAdder,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let engine = Engine::default();

    match cli.module.unwrap_or(Module::All) {
        Module::All => {
            run_adder_rs_sync(&engine)?;
            run_interfaced_adder_sync(&engine)?;
            run_adder_rs_async(&engine)?;
            run_adder_py_sync(&engine)?;
            run_adder_py_async(&engine)?;
            run_kv_store_rs_sync(&engine)?;
            run_kv_store_rs_async(&engine)?;
        }
        Module::RustAdder => {
            run_adder_rs_sync(&engine)?;
            run_interfaced_adder_sync(&engine)?;
            run_interfaced_adder_dynamic(&engine)?;
            run_adder_rs_async(&engine)?;
        }
        Module::RustKVStore => {
            run_kv_store_rs_sync(&engine)?;
            run_kv_store_rs_async(&engine)?;
        }
        Module::PythonAdder => {
            run_adder_py_sync(&engine)?;
            run_adder_py_async(&engine)?;
        }
    }

    println!("Run without errors!");
    Ok(())
}
