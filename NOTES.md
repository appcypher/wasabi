### ISSUES
- Replace llvm git clone in `wasmception` with downloading `.tar.gz`s from [llvm download site](https://releases.llvm.org/download.html). The clone thing is stupid slow.
- clang sometimes hangs when you don't specify `-Wl,--no-threads`, so this is included in wasm-specific flags passed to clang by default
