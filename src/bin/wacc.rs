#[macro_use]
extern crate wasabi;

use std::env;
use wasabi::common::{get_project_dir, get_project_bin_dir, run_command, get_combined_args};
use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};

fn main() -> Result<(), String> {
    let project_dir = get_project_dir();
    let project_bin_dir = get_project_bin_dir(&project_dir);

    let args: Vec<String> = env::args().skip(1).collect();
    let combined_args = get_combined_args(&project_dir, &args, None, true);

    let mut wasabi_subcommand_args = vec!["cc".to_string()];
    wasabi_subcommand_args.append(&mut args.clone());

    run_command(project_bin_dir, "wasabi", &wasabi_subcommand_args);

    debug!("[wacc]: args = {:?}", args);

    Ok(())
}
