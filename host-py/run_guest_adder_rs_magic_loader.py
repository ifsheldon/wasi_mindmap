# Reference: https://component-model.bytecodealliance.org/language-support/python.html#running-components-from-python-applications
from wasmtime import Store
# The magic, refer to https://github.com/bytecodealliance/wasmtime-py?tab=readme-ov-file#usage
import wasmtime.loader
from guest_adder_rs import Root

def run_adder_rs_guest():
    store = Store()
    adder_component_instance = Root(store)
    result = adder_component_instance.add(store, 1, 2)
    print(f"{__name__}: 1 + 2 = {result}")
    assert result == 3
