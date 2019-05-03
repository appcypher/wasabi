#[macro_use]
extern crate wasabi;

use std::env;
use wasabi::common::{get_project_dir, get_project_bin_dir, get_llvm_bin_dir, run_command, get_combined_args};
use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};

fn main() -> Result<(), String> {
    let project_dir = get_project_dir();
    let lvm_bin_dir = get_llvm_bin_dir(&project_dir);

    let wasabi_cli = get_wasabi_cli();
    let args: Vec<String> = env::args().skip(2).collect();
    let combined_args = get_combined_args(&project_dir, &args, None, true);

    match wasabi_cli.subcommand() {
        ("cc", Some(matches)) => {
            // run_command(lvm_bin_dir, "clang", &combined_args);
            run_command(lvm_bin_dir, "clang", &args);
        }
        (subcmd, _) => {
            return Err(format!("Unrecognized subcommand: {}", subcmd));
        }
    }

    debug!("[wasabi]: wasabi_cli = {:?}, args = {:?}", wasabi_cli, args);

    Ok(())
}

fn get_wasabi_cli<'a>() -> ArgMatches<'a> {
    App::new("wasabi")
        .version("0.0.0")
        .about("For compiling C/C++ projects to WebAssembly")
        .setting(AppSettings::AllowExternalSubcommands)
        .get_matches()
}

