use crate::PACKAGE_NAME;
use quote::quote;
use std::{collections::HashMap, fs, io::Write};

pub fn process(filename_to_st: &mut HashMap<String, syn::File>) {
    let pname = unsafe { PACKAGE_NAME };
    let ts = format!(
        "let mut metadata = std::fs::metadata(\"./lib{}.{}\").unwrap();",
        pname,
        crate::SUFFIX
    );
    let st: syn::Stmt = syn::parse_str(&ts).unwrap();
    let mut stv = vec![st];
    let ts = format!(
        r#"
      std::thread::spawn(move || loop {{
          std::thread::sleep(std::time::Duration::from_secs(10));
          let new_metadata = std::fs::metadata("./lib{}.{}").unwrap();
          if new_metadata.modified().unwrap() != metadata.modified().unwrap() {{
              let mut lib_guard = {}::LIB.write().unwrap();
              let lib = lib_guard.take().unwrap();
              lib.close().unwrap();
              *lib_guard = unsafe {{ Some(libloading::Library::new("./lib{}.{}").unwrap()) }};
              println!("lib changed");
              metadata = new_metadata;
          }}
      }});
    "#,
        pname,
        crate::SUFFIX,
        pname.replace("-", "_"),
        pname,
        crate::SUFFIX
    );
    let st: syn::Stmt = syn::parse_str(&ts).unwrap();
    stv.push(st);
    for (path, st) in filename_to_st {
        if path == "src/main.rs" {
            for item in &mut st.items {
                if let syn::Item::Fn(func) = item {
                    if func.sig.ident.to_string() == "main" {
                        stv.extend_from_slice(&func.block.stmts);
                        func.block.stmts = stv;
                        let stt = quote!(#st);
                        let mut new_file = fs::File::create(path).unwrap();
                        new_file.write_all(stt.to_string().as_bytes()).unwrap();
                        new_file.sync_data().unwrap();
                        return;
                    }
                }
            }
        }
    }
}
