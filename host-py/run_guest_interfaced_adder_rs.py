from wasmtime import Store
from interfaced_adder_rs_bindings import Root


def run_adder_rs_guest():
    store = Store()
    adder_component_instance = Root(store)
    result = adder_component_instance.add().add(store,1, 2)
    assert result == 3
    print(f"{__name__}: 1 + 2 = {result}")
