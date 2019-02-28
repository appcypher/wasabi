<div align="center">
    <a href="#" target="_blank">
        <img src="https://github.com/appcypher/wasabi/blob/master/media/wasabi.png" alt="Wasabi Logo" width="140" height="140"></img>
    </a>
</div>

<h1 align="center">WASABI</h1>

Wasabi is supposed to be a WebAssembly-focused way of generating wasm files from C/C++ code. Generated wasm files have an LLVM/wasm32 ABI.

### BUILDING THE PROJECT

#### BSD (macOS, ...) and Linux
- Clone the repository and its submodules.
    ```bash
    git clone --recurse-submodules https://github.com/appcypher/wasabi.git
    ```

    ```bash
    cd wasabi
    ```
- Wasabi is a [rust](https://www.rust-lang.org) project so it depends on `rustc` and `cargo`.

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

- Build `wasabi` and its dependencies.
    ```bash
    bash setup.sh install
    ```
    This command does the following:
    - clones the [`llvm`](https://llvm.org/) repository and builds it using steps defined in `wasmception`. This step may take a while.
    - clones `musl` repository.
    - builds the `wasabi` project.
    - installs necessary commands like `wacc`, `wa++` and `wasabi`.

- After a successful install you should be able to use the `setup.sh` script through the `wasabi` command.
    ```bash
    wasabi --help
    ```

- Use `wacc` or `wa++` command
    ```bash
    wacc test.c -o test.wasm
    ```

    ```bash
    wa++ test.cpp -o test.wasm -Wl,--export=func --gen=web
    ```


#### Windows
- N/A

### ATTRIBUTIONS
- [wasmception](https://github.com/yurydelendik/wasmception) - needed for installing llvm and getting libs/abis.
- [freepik](https://www.freepik.com/) - wasabi current logo
