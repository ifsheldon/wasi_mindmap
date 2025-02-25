1. Install tools: `pip install -r requirements.txt`
2. Make sure you have built `guest_adder_rs.wasm` and `guest_interfaced_adder_rs.wasm`. If not,
   `cd ../guest-adder-rs && cargo build --release --target wasm32-wasip2` and `cd ../guest-interfaced-adder-rs && cargo build --release --target wasm32-wasip2`
3. Make bindings for `guest_adder_rs.wasm` and `guest_interfaced_adder_rs.wasm`:
   `python -m wasmtime.bindgen guest_adder_rs.wasm --out-dir adder_rs_bindings && python -m wasmtime.bindgen guest_interfaced_adder_rs.wasm --out-dir interfaced_adder_rs_bindings`
4. Run the host: `python host.py`

## Limitations

According to
the [component model documentation](https://component-model.bytecodealliance.org/language-support/python.html#running-components-from-python-applications)

> wasmtime-py does not currently support running components build with componentize-py. This is because wasmtime-py does
> not yet support resources, which components built with componentize-py always use, since componentize-py
> unconditionally
> imports most of the wasi:cli world.

My experiments, which did the same steps to `guest_largestring_rs.wasm` and `guest_adder_py.wasm`, are also not
successful.