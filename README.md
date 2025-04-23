# Quantum Computing Demos in Rust & Python

A versatile playground for quantum-inspired computing, combining:

- **Rust + wasm-bindgen**: A WebAssembly module exposing a simple qubit API to JavaScript.  
- **Rust CLI**: A standalone binary demonstrating qubit operations and Python interop via PyO3.  
- **Python Qiskit Wrapper**: A helper script wrapping Qiskit circuits for real quantum simulation.

---

## Table of Contents

1. [Overview](#overview)  
2. [Features](#features)  
3. [Prerequisites](#prerequisites)  
4. [Installation & Build](#installation--build)  
   - [Rust CLI & Python Extension](#rust-cli--python-extension)  
   - [WebAssembly Module](#webassembly-module)  
5. [Configuration](#configuration)  
6. [Usage](#usage)  
   - [CLI Demo](#cli-demo)  
   - [Python Qiskit Integration](#python-qiskit-integration)  
   - [JavaScript/WebAssembly Demo](#javascriptwebassembly-demo)  
7. [Project Structure](#project-structure)  
8. [Extending & Customization](#extending--customization)  
9. [Troubleshooting](#troubleshooting)  
10. [Contributing](#contributing)  
11. [License](#license)  

---

## Overview

This repository shows you three ways to work with “qubits”:

1. **Rust WebAssembly**:  
   - Define a `Qubit` with amplitude fields  
   - Export via `wasm-bindgen` for use in browser JS  
2. **Rust CLI + PyO3**:  
   - A console app that creates and manipulates a qubit (Hadamard gate)  
   - Calls into a Python module (`qiskit_wrapper.py`) to run a real Qiskit circuit  
3. **Python Script**:  
   - `qiskit_wrapper.py` builds a 2-qubit Bell state and returns measurement counts  

---

## Features

- **Qubit Type**:  
  - `state_zero` and `state_one` amplitude fields  
  - `hadamard()` operation  
- **WASM Export**:  
  - `create_and_measure_qubit()` simulates measurement in JS  
- **PyO3 Extension**:  
  - Rust binary links into Python via `#[pyfunction]`/`maturin` or `cargo build`  
- **Qiskit Integration**:  
  - Python wrapper that uses Qiskit’s simulator  

---

## Prerequisites

- **Rust** (≥ 1.60) with `wasm32-unknown-unknown` target  
- **wasm-pack** (for WASM builds)  
- **Python** (≥ 3.8) with `qiskit` and `pyo3` build tool:  
  ```bash
  pip install qiskit
  # and for building the extension:
  pip install maturin
  ```  
- **Node.js** (for running JS+WASM demo)

---

## Installation & Build

### Rust CLI & Python Extension

```bash
# Clone
git clone https://github.com/nicktretyakov/quantum.git
cd quantum

# Build the PyO3 extension (with maturin)
maturin develop --release

# Or build just the CLI:
cargo build --release
```

This produces:

- A `quantum` binary in `target/release/`  
- A Python module `quantum` you can `import` in Python  

### WebAssembly Module

```bash
# From repo root
cd src
wasm-pack build --target web --release
```

This generates a `pkg/` directory containing:

- `rust_bg.wasm`  
- `rust.js` glue code  

You can then `import * as rust from './pkg/rust.js'` in a web page.

---

## Configuration

No configuration files are needed.  
- The Python module `qiskit_wrapper.py` lives in project root.  
- JS demo simply loads the WASM package.

---

## Usage

### CLI Demo

```bash
# Run the Rust binary
./target/release/quantum
```

**Sample Output:**
```
Исходное состояние кубита: |0⟩ = 1.0, |1⟩ = 0.0
Состояние кубита после операции Адамара: |0⟩ = 0.7071, |1⟩ = 0.7071
Результаты измерений: {'00': 510, '11': 490}
```

### Python Qiskit Integration

From any Python REPL:

```python
import qiskit_wrapper

counts = qiskit_wrapper.run_quantum_circuit()
print("Counts:", counts)
```

This builds a Bell state (Hadamard + CNOT) and runs 1 000 shots on Qiskit’s `qasm_simulator`.

### JavaScript/WebAssembly Demo

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8" />
  <script type="module">
    import init, { create_and_measure_qubit } from './pkg/rust.js';

    async function run() {
      await init();
      const outcome = create_and_measure_qubit();
      document.body.textContent = outcome;
    }

    run();
  </script>
</head>
<body>
  Loading…
</body>
</html>
```

- Compile with `wasm-pack`  
- Serve via any static HTTP server (e.g. `npx serve .`)

---

## Project Structure

```
quantum/
├── Cargo.toml           # Rust package (name = "rust")
├── .gitignore
├── qiskit_wrapper.py    # Python Qiskit demo
├── Cargo.lock
└── src/
    ├── lib.rs           # wasm-bindgen module
    └── main.rs          # CLI + PyO3 interop demo
```

---

## Extending & Customization

- **Additional Gates**: Implement `X`, `Z`, or multi-qubit gates in Rust/lib.rs  
- **Measurement**: Expose more WASM functions for amplitudes or probabilities  
- **Real Devices**: Swap Qiskit’s simulator for IBMQ backends in `qiskit_wrapper.py`  
- **Package Publishing**: Use `maturin publish` to upload the Python wheel to PyPI  

---

## Troubleshooting

- **WASM Missing**: Ensure `wasm-pack` adds `pkg/`; check your HTTP server MIME types  
- **Python Build Errors**:  
  - Install matching Python dev headers  
  - Use `maturin develop --release` or `cargo build --release` with `--features pyo3/extension-module`  
- **Camera/Quantum**: Not applicable—no hardware GPU or camera needed  

---

## Contributing

1. **Fork** this repo  
2. **Implement** new gates, demos, or language bindings  
3. **Test** your code:  
   - Rust: `cargo test`  
   - Python: write a small script that imports `qiskit_wrapper`  
4. **Submit** a PR with clear description  

---

## License

This project is licensed under the **MIT License**.  
See [LICENSE](LICENSE) for details.
