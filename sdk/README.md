Useful arguments:
  - `--sysroot=../sysroot`: gives the sysroot path (with libc + abi)
  - (`-g`): debug symbols
  - `-O3`: optimize, optimize, optimize !
  - `-o main.wasm`: output file
  - `-nostartfiles`: tell to CLang we don't have any `main` function
  - `-fvisibility=hidden`: tell to CLang all symbols are hidden by default
  - `-Wl`: pass to wasm linker the following arguments:
    - `--allow-undefined-file=main.syms`: allow to "link" some undefined symbols, they will be imported from JS
    - `--import-memory`: let us define the `WebAssembly.Memory` object from JS
    - `--import-table`: we also want to define ourself the `WebAssembly.Table` object from JS
    - (`--strip-all`): remove useless symbols
    - (`--demangle`): useful if we build and export function in C++
    - `--no-entry`: tell to the linker we don't have any entry point (`__start` function)
    - (`--no-threads`): linking with thread support doesn't work well on every arch

