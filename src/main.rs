use clap::Parser;
use std::{env, path::Path, process::Command};

mod file_collector;
mod lib_builder;
mod mod_handler;
mod thread_injector;
mod toml_handler;
mod unsafe_block_handler;
mod unsafe_fn_handler;

#[cfg(target_os = "macos")]
static SUFFIX: &str = "dylib";
#[cfg(target_os = "linux")]
static SUFFIX: &str = "so";
#[cfg(target_os = "windows")]
static SUFFIX: &str = "lib";

static mut PACKAGE_NAME: &str = "";

#[derive(Parser, Debug)]
struct Args {
    /// rust project directory
    #[arg(value_parser = is_dir)]
    directory: String,

    /// inject thread
    #[arg(short, default_value_t = false)]
    i: bool,
}

fn is_dir(s: &str) -> Result<String, String> {
    if !Path::new(s).exists() {
        return Err(format!("{} isn't a valid directory", s));
    }
    Ok(s.to_owned())
}

fn main() {
    let args = Args::parse();
    env::set_current_dir(&args.directory).unwrap();
    toml_handler::process();
    let mut filename_to_st = file_collector::process();
    let (filename_to_unsafe_fn_name, unsafe_fn_name_to_sig) =
        unsafe_fn_handler::process(&mut filename_to_st);
    // Back up the publicity attribute of the mod
    let mut filename_to_st_backup = filename_to_st.clone();
    mod_handler::process(&mut filename_to_st_backup);
    lib_builder::process(&filename_to_unsafe_fn_name);
    unsafe_block_handler::process(&unsafe_fn_name_to_sig, &mut filename_to_st);
    if args.i {
        thread_injector::process(&mut filename_to_st);
    }
    Command::new("cargo").arg("fmt").output().unwrap();
}
