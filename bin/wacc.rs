use std::{
    env,
    process::Command,
};

const WACC_ARGS: [&str; 1] = [
    "--gen=web"
];

fn main() {
    println!("\n=== [ wacc command says hi! ] ===\n");

    // Get arguments.
    let mut args: Vec<String> = env::args().collect();

    // Strip `wacc` specific arguments. TODO
    for (index, arg) in WACC_ARGS.iter().enumerate() {
        if WACC_ARGS.contains(arg) {
            args.remove(index);
        }
    }

    // Prepare args for clang.
    let clang_args = args.join(" ");

    // Create command.
    let mut child = Command::new("wacc") // TODO: `wacc` To be replaced here with path to wasmception-prepared clang
        .arg(clang_args)
        .spawn()
        .expect("Failed to invoke `clang`");

    child.wait().expect("Failed to wait on child");

    println!("\n=== [ wacc command says hi! ] ===\n");
}
