### WHAT IS WASABI?
- Wasabi is a `wasm32-llvm-linux` project.
- Wasabi is forward-facing and wasm-focused.
- Wasabi will support wasm-web interop via hostbindings apis.

### WINDOWS SUPPORT
- Statically analyze syscalls that are embedded in code.
- Creating a detailed `mapping structure`.
- Some examples of POSIX API
    ```
    [ FILES ]
    - open
    - openat
    - close

    [ NETWORK ]
    - socket
    - setsockopt

    [ DYNAMIC LIBRARY ]
    - dlopen
    ```

- CAVEATS
    - Separated continuation like `select/epoll`
    - Unmappable syscalls like `ioctl`



### ISSUES
- Replace llvm git clone in `wasmception` with downloading `.tar.gz`s from [llvm download site](https://releases.llvm.org/download.html). The clone thing is stupid slow.

- `clang` sometimes hangs when you don't specify `-Wl,--no-threads`, so this is included in wasm-specific flags passed to clang by default. This will be removed once threads have been properly supported.

- For a simple `printf hello world` test, `emcc` generates a 131-line wat file, while the `wacc` clang (v9.0) generates a 1778-line file. At first, I thought emscripten was performing certain wasm-specific optimizations through `binaryen` but it turns out emscripten imports the `printf` function. llvm-wasm backend, on the other hand, implements the entire thing inside the wasm module. I've not tested their performances but I think emscripten made the right call here. Printf is probably one of the most-used C function. Bundling it with a wasm module every time is kinda wasteful.
The emcc-generated wasm file takes up 799 bytes while the wacc-generated one takes up 39Kb.
