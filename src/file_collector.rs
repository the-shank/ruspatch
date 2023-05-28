//! Find all .rs files and generate a mapping from file relative path (src/...) to file syntax tree
use std::{collections::HashMap, fs};
use walkdir::WalkDir;

/// Find all .rs files in the project directory
/// FIXME: Unable to distinguish whether a found file is associated with a project

pub fn process() -> HashMap<String, syn::File> {
    let mut res = HashMap::new();
    for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
        let f_path = entry.path().to_string_lossy();
        if f_path.ends_with(".rs") {
            let input_code =
                fs::read_to_string(&*f_path).expect(&format!("cannot read {}", f_path));
            let st = syn::parse_file(&input_code).expect(&format!("cannot parse {}", f_path));
            res.insert(f_path.to_string(), st);
        }
    }
    res
}
