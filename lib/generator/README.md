## THE GENERATOR
These are all ideas on how the final system could look like. Everything is still very much a work in progress.

### wacc / wa++
`wacc` and `wa++` commands are supposed to serve as a drop-in replacement for `clang` and `clang++` respectively. They contain code needed for handling `wasabi`-specific arguments as well as passing proper flags to `clang` for wasm compilation.

One of `wasabi`-specific arguments is the `--gen` flag. It generates the necessary bindgen for running in respective runtime.

- Generating js wrapper code.

    ```bash
    wacc test.c -o test.wasm --gen=web
    ```
    
- Generating wasmer wrapper code. It also determines the host data needed and creates a cargo project that can run the wasm file.

    ```bash
    wa++ test.cpp -o test.wasm --gen=wasmer
    ```

- Generating nodejs wrapper code.

    ```bash
    wacc test.c -o test.wasm --gen=node
    ```

### refs
1. Compilation Flags
    - https://emscripten.org/docs/tools_reference/emcc.html
