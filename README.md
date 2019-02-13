# wasabi (another way of doing things)
Wasabi is supposed to be a WebAssembly-focused way of generating wasm files from C/C++ code.

## BUILDING THE PROJECT
#### BSD (macOS, ...) and Linux
- Clone the repositry and its submodules
    ```bash
    git clone --recurse-modules https://github.com/appcypher/wasabi.git
    ```

    ```bash
    cd wasabi
    ````
- Build the `wasmception` projec and its dependencies.
    ```bash
    sudo sh setup.sh install
    ```

    This should install necessary commands like `wacc`, `wa++` and `wasabi`.

- After a successful install you should be able to use the `setup.sh` script via the wasabi command.
    ```bash
    wasabi install
    ```

    ```bash
    wasabi uninstall
    ```



#### Windows
- N/A

### ATTRIBUTIONS
- [wasmception](https://github.com/yurydelendik/wasmception) - needed for getting and necessary lib and
- freepik -
