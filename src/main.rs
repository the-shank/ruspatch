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
pub static SUFFIX: &str = "dylib";
#[cfg(target_os = "linux")]
pub static SUFFIX: &str = "so";
#[cfg(target_os = "windows")]
pub static SUFFIX: &str = "lib";

static mut PACKAGE_NAME: Option<String> = None;

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
    // 给Cargo.toml添加依赖。
    toml_handler::process();
    // 找出所有.rs文件，生成文件相对路径（src/...）到文件语法树的映射。
    let mut filename_to_st = file_collector::process();
    // 将文件语法树中所有的unsafe函数放在文件的最外层（不放入任何块内），生成映射[文件相对路径 -> 文件包含的unsafe函数列表]和
    // [unsafe函数名 -> unsafe函数的参数和返回值类型）]，同时给unsafe函数添加pub和#[no_mangle]标签。
    let (filename_to_unsafe_fn_name, unsafe_fn_name_to_sig) =
        unsafe_fn_handler::process(&mut filename_to_st);
    // 为了最终写回文件中的mod的publicity属性保持原样，在此处备份。
    let mut filename_to_st_backup = filename_to_st.clone();
    // 将文件语法树中所有的mod改为pub mod，并写回文件。
    mod_handler::process(&mut filename_to_st_backup);
    // 编译动态链接库。
    lib_builder::process(&filename_to_unsafe_fn_name);
    // 将文件语法树中所有的unsafe函数调用改为对动态链接库中函数的调用。
    unsafe_block_handler::process(&unsafe_fn_name_to_sig, &mut filename_to_st);
    if args.i {
        // 执行线程注入。
        thread_injector::process(&mut filename_to_st);
    }
    Command::new("cargo").arg("fmt").output().unwrap();
}
