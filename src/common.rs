use std::env;
use std::process::Command;

pub fn get_project_dir() -> String {
    let mut path_to_exe = env::current_exe().unwrap();
    path_to_exe.push("../../../");

    path_to_exe
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn get_sysroot_flag(project_dir: &str) -> String {
    format!("--sysroot={}{}", project_dir, "/deps/wasmception/sysroot")
}

pub fn get_project_bin_dir(project_dir: &String) -> String {
    format!("{}/target/debug", project_dir)
}

pub fn get_llvm_bin_dir(project_dir: &String) -> String {
    format!("{}/deps/wasmception/dist/bin", project_dir)
}

pub fn get_clang_wasm_flags(exports: Option<Vec<String>>, is_executable: bool) -> Vec<String> {
    let export_flag = match exports {
        Some(exports) => format!(
            "-W,{}",
            exports
                .iter()
                .map(|s| format!("--export={}", s))
                .collect::<Vec<String>>()
                .join(",")
        ),
        None => String::new(),
    };

    let main_flag = if is_executable {
        String::from("-Wl,--entry=main,--export=main")
    } else {
        String::new()
    };

    vec![
        // TODO: Link in reference to printf, sprintf. To be implemented by VM.
        // No system startup files. No _start, _ini, _fini
        String::from("-nostartfiles"),
        // Clang currenly crashes with threads enabled.
        String::from("-Wl,--no-threads"),
        // Ideally malloc and co should be exported.
        String::from("-Wl,--export=malloc"),
        // Main flag, if there is any.
        main_flag,
        // Export flag if there are any exports.
        export_flag,
    ]
}

pub fn get_combined_args(
    project_dir: &str,
    args: &[String],
    exports: Option<Vec<String>>,
    is_executable: bool,
) -> Vec<String> {
    let mut combined_args = vec![];
    let mut clang_wasm_flags = get_clang_wasm_flags(exports, is_executable);
    let sysroot_flag = get_sysroot_flag(project_dir);
    let mut args = args.to_vec().clone();

    combined_args.append(&mut clang_wasm_flags);
    combined_args.append(&mut vec![sysroot_flag]);
    combined_args.append(&mut args);

    combined_args
}

pub fn run_command(bin_dir: String, command: &str, args: &[String]) {
    let command_path = format!("{}/{}", bin_dir, command);

    debug!("command_path = {:?}", command_path);

    let mut child = Command::new(&command_path)
        .args(args)
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to invoke {}", command));

    child.wait().expect("Failed to wait on child");
}

pub fn filter_wasabi_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut clang_args = vec![];
    let mut wasabi_args = vec![];
    // let valid_wasabi_args = vec![];

    (clang_args, wasabi_args)
}
