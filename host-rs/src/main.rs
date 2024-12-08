use crate::adder::*;
use crate::large_string::*;
use clap::{Parser, ValueEnum};
use wasmtime::{Config, Engine};

mod adder;
mod large_string;
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
    RustLargeString,
    PythonAdder,
}

fn main() {
    let cli = Cli::parse();

    // engines with/without async must be used separately for async and sync instantiation of components
    let engine_sync = Engine::default();
    let mut config = Config::default();
    config.async_support(true);
    let engine_async = Engine::new(&config).unwrap();

    match cli.module.unwrap_or(Module::All) {
        Module::All => {
            run_adder_rs_sync(&engine_sync);
            run_interfaced_adder_sync(&engine_sync);
            run_adder_rs_async(&engine_async);
            run_adder_py_sync(&engine_sync);
            run_adder_py_async(&engine_async);
            run_large_string_rs_sync(&engine_sync);
            run_large_string_rs_async(&engine_async);
        }
        Module::RustAdder => {
            run_adder_rs_sync(&engine_sync);
            run_interfaced_adder_sync(&engine_sync);
            run_adder_rs_async(&engine_async);
        }
        Module::RustLargeString => {
            run_large_string_rs_sync(&engine_sync);
            run_large_string_rs_async(&engine_async);
        }
        Module::PythonAdder => {
            run_adder_py_sync(&engine_sync);
            run_adder_py_async(&engine_async);
        }
    }

    println!("Run without errors!");
}
