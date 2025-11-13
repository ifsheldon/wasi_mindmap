Reference: https://component-model.bytecodealliance.org/language-support/python.html

1. Install tools: `uv sync`
2. Generate bindings:
   `uv run poe bind-adder && uv run poe bind-interfaced-adder`
3. Implement the guest: create [`guest-adder.py`](guest-adder.py) and [`guest-interfaced-adder.py`](guest-interfaced-adder.py)
4. Componentize the guest:
   `uv run poe componentize-adder && uv run poe componentize-interfaced-adder`
