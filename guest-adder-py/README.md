Reference: https://component-model.bytecodealliance.org/language-support/python.html

1. Install tools: `pip install componentize-py`
2. Generate bindings: `componentize-py --wit-path ../wit-files/adder.wit --world adder bindings .`
3. Implement the guest: create [`guest-adder.py`](guest-adder.py)
4. Componentize the guest:
   `componentize-py --wit-path ../wit-files/adder.wit --world adder componentize guest-adder -o guest_adder_py.wasm`