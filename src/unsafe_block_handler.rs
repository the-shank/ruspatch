use crate::PACKAGE_NAME;
use quote::quote;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
};
use syn::visit_mut::{self, VisitMut};

struct UnsafeBlockHandler<'a> {
    unsafe_fn_name_to_sig: &'a HashMap<String, (String, String)>,
    in_unsafe_block: bool,
    unsafe_calls: HashSet<String>,
}

impl UnsafeBlockHandler<'_> {
    fn new(unsafe_fn_name_to_sig: &HashMap<String, (String, String)>) -> UnsafeBlockHandler {
        UnsafeBlockHandler {
            unsafe_fn_name_to_sig,
            in_unsafe_block: false,
            unsafe_calls: HashSet::new(),
        }
    }
}

impl VisitMut for UnsafeBlockHandler<'_> {
    fn visit_expr_unsafe_mut(&mut self, i: &mut syn::ExprUnsafe) {
        self.in_unsafe_block = true;
        visit_mut::visit_expr_unsafe_mut(self, i);
        let pname = unsafe { PACKAGE_NAME };
        let ts = format!(
            "let lib_guard = {}::LIB.read().unwrap();",
            pname.replace("-", "_")
        );
        let st: syn::Stmt = syn::parse_str(&ts).unwrap();
        let mut stv = vec![st];
        let ts = "let lib = lib_guard.as_ref().unwrap();";
        let st: syn::Stmt = syn::parse_str(&ts).unwrap();
        stv.push(st);
        for c in &self.unsafe_calls {
            let args = self.unsafe_fn_name_to_sig[c.as_str()].0.as_str();
            let ret = self.unsafe_fn_name_to_sig[c.as_str()].1.as_str();
            let ts = format!(
                "let {}:libloading::Symbol<fn({}){}> = lib.get(b\"{}\").unwrap();",
                c, args, ret, c
            );
            let st: syn::Stmt = syn::parse_str(&ts).unwrap();
            stv.push(st);
        }
        stv.extend_from_slice(&i.block.stmts);
        i.block.stmts = stv;
        self.unsafe_calls.clear();
        self.in_unsafe_block = false;
    }

    fn visit_expr_path_mut(&mut self, i: &mut syn::ExprPath) {
        if self.in_unsafe_block {
            let fn_name = i.path.segments.last().unwrap().ident.to_string();
            if self.unsafe_fn_name_to_sig.contains_key(&fn_name)
                && !self.unsafe_calls.contains(&fn_name)
            {
                self.unsafe_calls.insert(fn_name);
            }
        }
        visit_mut::visit_expr_path_mut(self, i);
    }
}

/// 将unsafe代码块改为对unsafe函数的调用
pub fn process(
    unsafe_fn_name_to_sig: &HashMap<String, (String, String)>,
    filename_to_st: &mut HashMap<String, syn::File>,
) {
    let mut handler = UnsafeBlockHandler::new(unsafe_fn_name_to_sig);
    for (path, st) in filename_to_st {
        if path != "src/lib.rs" {
            handler.visit_file_mut(st);
            let stt = quote!(#st);
            let mut new_file = fs::File::create(path).unwrap();
            new_file.write_all(stt.to_string().as_bytes()).unwrap();
            new_file.sync_data().unwrap();
        }
    }
}
