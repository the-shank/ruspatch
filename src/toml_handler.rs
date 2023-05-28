//! Add dependencies to Cargo.toml
use std::{fs, io::Write};
use toml::Table;

use crate::PACKAGE_NAME;

pub fn process() {
    let tomlstr = fs::read_to_string("Cargo.toml").unwrap();
    let mut toml = tomlstr.parse::<Table>().unwrap();
    if !toml.contains_key("dependencies") {
        toml.insert(
            "dependencies".to_owned(),
            toml::Value::Table(toml::Table::new()),
        );
    }

    // PACKAGE_NAME
    unsafe {
        PACKAGE_NAME = Box::leak(
            toml["package"]["name"]
                .as_str()
                .unwrap()
                .to_owned()
                .into_boxed_str(),
        );
    }

    let dep = toml["dependencies"].as_table_mut().unwrap();
    if !dep.contains_key("libloading") {
        dep.insert("libloading".to_owned(), "0.7".into());
    }
    if !dep.contains_key("lazy_static") {
        dep.insert("lazy_static".to_owned(), "1.4".into());
    }

    let mut tomlfile_new = fs::File::create("Cargo.toml").unwrap();
    tomlfile_new.write_all(toml.to_string().as_bytes()).unwrap();
    tomlfile_new.sync_data().unwrap();
}
