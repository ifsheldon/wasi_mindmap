Reference: https://component-model.bytecodealliance.org/language-support/python.html

1. Install tools: `pip install -r requirements.txt`
2. Generate bindings:
   `componentize-py --wit-path ../wit-files/adder.wit --world adder bindings . && componentize-py --wit-path ../wit-files/interfaced-adder.wit --world adder bindings . --world-module interfaced_adder`
3. Implement the guest: create [`guest-adder.py`](guest-adder.py) and [`guest-interfaced-adder.py`](guest-interfaced-adder.py)
4. Componentize the guest:
   `componentize-py --wit-path ../wit-files/adder.wit --world adder componentize guest-adder -o guest_adder_py.wasm && componentize-py --wit-path ../wit-files/interfaced-adder.wit --world adder componentize guest-interfaced-adder -o guest_interfaced_adder_py.wasm`