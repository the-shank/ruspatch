use std::{collections::HashMap, fs};
use walkdir::WalkDir;

/// 找出项目目录中所有的.rs文件。
/// !!!无法区分找到的文件是否与项目有关联!!!。

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
