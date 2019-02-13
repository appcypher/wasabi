//!

use std::{
    env,
    process::Command,
};

use wasabi::common::WASABI_SPECIFIC_ARGS;

fn main() {
    println!("\n=== [ wacc ] ===\n");

    // Get arguments.
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    // Remove `wacc` specific arguments. TODO
    // let clang_args = args.iter().filter(|arg| WASABI_SPECIFIC_ARGS.contains(arg)).collect();

    for (index, arg) in WASABI_SPECIFIC_ARGS.iter().enumerate() {
        println!("index = {:?}", index);
        // if WASABI_SPECIFIC_ARGS.contains(arg) {
        //     args.remove(index);
        // }
    }

    println!("KABBOM", );

    // Prepare args for clang.
    let clang_args = args.join(" ");

    // Create command.
    // TODO: `wacc` To be replaced here with path to wasmception-prepared clang
    let mut child = Command::new("wacc-test")
        .arg(clang_args)
        .spawn()
        .expect("Failed to invoke `clang`");

    child.wait().expect("Failed to wait on child");

    println!("\n=== [ wacc ] ===\n");
}
