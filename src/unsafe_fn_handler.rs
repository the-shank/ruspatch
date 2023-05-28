//! Put all unsafe functions in the outermost layer of the file (not in any block), 
//! generate mapping [file relative path -> list of unsafe functions contained in the file] and
//! [unsafe function name -> parameter and return value type of unsafe function], 
//! add pub and #[no_mangle] tags to the unsafe function.


use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::Parser;
use syn::visit_mut::{self, VisitMut};

// Move all unsafe functions to the outermost layer.

struct UnsafeFnHandler {
    unsafe_fns: Vec<syn::ItemFn>,
}

impl UnsafeFnHandler {
    fn new() -> Self {
        UnsafeFnHandler { unsafe_fns: vec![] }
    }
}

impl VisitMut for UnsafeFnHandler {
    fn visit_item_mut(&mut self, i: &mut syn::Item) {
        take_mut::take(i, |i| match i {
            syn::Item::Fn(mut func) => {
                if func.sig.unsafety.is_some() {
                    // add "pub"
                    func.vis = syn::Visibility::Public(syn::token::Pub::default());
                    // add "#[no_mangle]"
                    let tokens = quote!(#[no_mangle]);
                    let mut att = syn::Attribute::parse_outer.parse2(tokens).unwrap();
                    func.attrs.append(&mut att);
                    self.unsafe_fns.push(func);
                    syn::Item::Verbatim(proc_macro2::TokenStream::new())
                } else {
                    syn::Item::Fn(func)
                }
            }
            _ => i,
        });
        visit_mut::visit_item_mut(self, i);
    }
}

pub fn process(
    filename_to_st: &mut HashMap<String, syn::File>,
) -> (
    HashMap<String, Vec<String>>,
    HashMap<String, (String, String)>,
) {
    copy_main(filename_to_st);
    let mut res1 = HashMap::new();
    let mut res2 = HashMap::new();
    for (path, st) in filename_to_st {
        if path != "src/main.rs" {
            let mut handler = UnsafeFnHandler::new();
            handler.visit_file_mut(st);
            for unsafe_fn in handler.unsafe_fns {
                let fn_name = unsafe_fn.sig.ident.to_string();
                let t = res1.entry(path.clone()).or_insert(vec![]);
                t.push(fn_name.clone());
                let targs = unsafe_fn.sig.inputs.to_token_stream().to_string();
                let tret = unsafe_fn.sig.output.to_token_stream().to_string();
                res2.insert(fn_name, (targs, tret));
                st.items.push(syn::Item::Fn(unsafe_fn));
            }
        }
    }
    (res1, res2)
}

// Copy main.rs to lib.rs
fn copy_main(filename_to_st: &mut HashMap<String, syn::File>) {
    filename_to_st.insert(
        "src/lib.rs".to_owned(),
        filename_to_st["src/main.rs"].clone(),
    );
    let lib_st = filename_to_st.get_mut("src/lib.rs").unwrap();
    lib_st.items.retain(|item| match item {
        syn::Item::Fn(func) => {
            if func.sig.ident.to_string() == "main" {
                false
            } else {
                true
            }
        }
        _ => true,
    });
}
