use std::process::Command;

pub const WASABI_SPECIFIC_ARGS: [&str; 4] =
    ["--gen=web", "--gen=node", "--gen=wasmer", "--gen=wasmlite"];

pub fn get_clang_wasm_flags() -> Vec<String> {
    vec![
        String::from("-nostartfiles"),
        // TODO: --no-threads is here temporarily to stop clang from crashing sometimes.
        // TODO: Add flag option to determine if entry is needed. Can be a side_module as in Emscripten.
        String::from("-Wl,--entry=main,--export=main,--export=malloc,--no-threads"),
    ]
}

pub fn get_clangxx_wasm_flags() -> Vec<String> {
    vec![
        String::from("-nostartfiles"),
        // TODO: --no-threads is here temporarily to stop clang from crashing sometimes.
        // TODO: Add flag option to determine if entry is needed. Can be a side_module as in Emscripten.
        String::from("-Wl,--entry=main,--export=main,--export=malloc,--no-threads"),
        // TODO: -fno-exceptions is added by default for now.
        String::from("-fno-exceptions"),
    ]
}

pub fn call_clang(command: String, clang_args: Vec<String>) {
    let mut child = Command::new(&command)
        .args(clang_args)
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to invoke {}", command));

    child.wait().expect("Failed to wait on child");
}
