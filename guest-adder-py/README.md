Reference: https://component-model.bytecodealliance.org/language-support/python.html

Run these commands from the `guest-adder-py` directory.
`uv sync` installs this example's development tools, including `poe` and `componentize-py`.

1. Install tools: `uv sync`
2. Generate bindings:
   `uv run poe bind-adder && uv run poe bind-interfaced-adder`
3. Implement the guest: create [`guest-adder.py`](guest-adder.py) and [`guest-interfaced-adder.py`](guest-interfaced-adder.py)
4. Componentize the guest:
   `uv run poe componentize-adder && uv run poe componentize-interfaced-adder`

> To see the details of the commands, see the poe task definitions in [pyproject.toml](pyproject.toml).
