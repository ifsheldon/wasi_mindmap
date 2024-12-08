from wasmtime import Store
from adder_rs_bindings import Root


def run_adder_rs_guest():
    store = Store()
    demo = Root(store)
    result = demo.add(store,1, 2)
    assert result == 3
    print("1 + 2 = ", result)
