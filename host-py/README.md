1. Install tools: `uv sync`
2. Make sure you have built `guest_adder_rs.wasm` and `guest_interfaced_adder_rs.wasm`. If not, run
   ```
   uv run poe build-adder-rs
   uv run poe build-interfaced-adder-rs
   ```
3. Make bindings for `guest_adder_rs.wasm` and `guest_interfaced_adder_rs.wasm`:
   ```
   uv run poe bind-adder-rs
   uv run poe bind-interfaced-adder-rs
   ```
4. Run the host: `uv run python host.py`

> To see the details of the commands, see the poe task definitions in [pyproject.toml](pyproject.toml).

## Limitations

`wasmtime-py` does not currently support running components build with componentize-py. This is because wasmtime-py does not yet support resources, which components built with componentize-py always use, since componentize-py unconditionally imports most of the wasi:cli world.

See [the issue](https://github.com/bytecodealliance/wasmtime-py/issues/309).
