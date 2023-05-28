//! Compile the dynamic link library

use crate::PACKAGE_NAME;
use quote::quote;
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;
use std::{env, fs};

// TODO: Avoid repeated uses and functions with the same name
pub fn process(filename_to_unsafe_fn_name: &HashMap<String, Vec<String>>) {
    Command::new("cargo")
        .args(["new", "temp_crate", "--lib"])
        .output()
        .unwrap();
    env::set_current_dir("temp_crate").unwrap();
    let mut st = syn::parse_file("").unwrap();
    // Avoid duplicating mods
    let mut toml_file = fs::OpenOptions::new()
        .append(true)
        .open("Cargo.toml")
        .unwrap();
    let pname = unsafe { PACKAGE_NAME };
    toml_file
        .write_all(
            format!(
                "{} = {{ path=\"..\" }}\n\n[lib]\ncrate-type = [\"cdylib\"]\n\n[workspace]\n",
                pname
            )
            .as_bytes(),
        )
        .unwrap();
    toml_file.sync_data().unwrap();

    for (file_path, fns) in filename_to_unsafe_fn_name {
        let mut divs = file_path.split('/').skip(1).collect::<Vec<_>>();
        let len = divs.len();
        divs[len - 1] = &divs[len - 1][..divs[len - 1].len() - 3];

        if len > 1 && (divs[len - 1] == "mod" || divs[len - 1] == divs[len - 2]) {
            divs.pop();
        }
        if file_path == "src/lib.rs" {
            divs.pop();
        }
        for func in fns {
            let mut t = "".to_string();
            for s in &divs {
                t += s;
                t += "::";
            }
            t += func;
            let stm = format!("pub use {}::{};", pname.replace("-", "_"), t);
            let sstm: syn::ItemUse = syn::parse_str(&stm).unwrap();
            st.items.push(syn::Item::Use(sstm));
        }
    }
    let stt = quote!(#st);
    let mut lib_file = fs::File::create("src/lib.rs").unwrap();
    lib_file.write_all(stt.to_string().as_bytes()).unwrap();
    lib_file.sync_data().unwrap();

    /*
    let out = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();
    let mut output = fs::File::create("../output.txt").unwrap();
    output.write_all(&out.stderr).unwrap();
    */
    Command::new("cargo")
        .args(["build"])
        .output()
        .expect("building temp_crate failed");
    env::set_current_dir("..").unwrap();
    fs::rename(
        format!("./temp_crate/target/debug/libtemp_crate.{}", crate::SUFFIX),
        format!("./lib{}.{}", pname, crate::SUFFIX),
    )
    .unwrap();
    // fs::remove_dir_all("./temp_crate").unwrap();
    build_libloading();
}

fn build_libloading() {
    let mut st = syn::parse_file("").unwrap();
    let ts = "use lazy_static::lazy_static;";
    let item: syn::Item = syn::parse_str(ts).unwrap();
    st.items.push(item);
    let pname = unsafe { PACKAGE_NAME };
    let ts = format!("lazy_static! {{ pub static ref LIB: std::sync::RwLock<Option<libloading::Library>> = unsafe {{ std::sync::RwLock::new(Some(libloading::Library::new(\"./lib{}.{}\").unwrap())) }};}}", pname, crate::SUFFIX);
    let item: syn::Item = syn::parse_str(&ts).unwrap();
    st.items.push(item);
    let stt = quote!(#st);
    let mut lib_file = fs::File::create("src/lib.rs").unwrap();
    lib_file.write_all(stt.to_string().as_bytes()).unwrap();
    lib_file.sync_data().unwrap();
}
