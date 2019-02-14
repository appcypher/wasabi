//! This file manages arguments that are expected to be passed to the clang command.

#![macro_use]
use wasabi::macros;

use std::env;

use wasabi::common::{call_clang, get_clang_wasm_flags, WASABI_SPECIFIC_ARGS};

fn main() {
    println!("\n=== [ wacc ] ===\n");

    // Get all arguments.
    let args: Vec<String> = env::args().collect();

    // Stores wasabi-specific args.
    let mut wasabi_args = vec![];

    // Stores clang args.
    let mut clang_args = vec![];

    // Get project root dir
    let mut path_to_exe = env::current_exe().unwrap();
    path_to_exe.push("../../../");

    let project_root_dir = path_to_exe
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    println!("project_root_dir = {:#?}\n", project_root_dir);

    // Used to determine if command is a compilation rather than `wacc --help` for example
    let mut is_compile_command = false;

    // Separate wasabi-specific args from clang args.
    for arg in &args[1..] {
        if WASABI_SPECIFIC_ARGS.contains(&arg.as_str()) {
            wasabi_args.push(arg.clone());
        } else {
            clang_args.push(arg.clone());
            // Check if arg is a output flag.
            if arg == "-o" {
                is_compile_command = true;
            }
        }
    }

    // Check if command is for compilation.
    if is_compile_command {
        // Get sysroot dir // TODO
        let sysroot_flag = format!(
            "--sysroot={}{}",
            project_root_dir, "/deps/wasmception/sysroot"
        );

        // Append sysroot dir
        clang_args.push(sysroot_flag);

        // NOTE: Had to append the wasm-specific flags after sysroot flags, otherwise
        // they may not acknowledged.
        // Append wasm-specific flags
        clang_args.append(&mut get_clang_wasm_flags());
    }

    println!("clang_args = {:#?}\n", clang_args);
    println!("wasabi_args = {:#?}\n", wasabi_args);

    // Command
    let command = format!(
        "{}{}",
        project_root_dir, "/deps/wasmception/dist/bin/clang"
    );

    // Call clang command
    call_clang(command, clang_args);

    println!("\n=== [ {} ] ===", args.join(" "));
    println!("\n=== [ wacc ] ===\n");
}
