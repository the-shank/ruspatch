//! Change all mods in the file syntax tree to pub mods

use quote::quote;
use std::{collections::HashMap, fs, io::Write};
use syn::visit_mut::{self, VisitMut};

struct ModHandler;

impl VisitMut for ModHandler {
    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        i.vis = syn::Visibility::Public(syn::token::Pub::default());
        visit_mut::visit_item_mod_mut(self, i);
    }
}

pub fn process(filename_to_st: &mut HashMap<String, syn::File>) {
    for (path, st) in filename_to_st {
        if path != "src/main.rs" {
            let mut mod_handler = ModHandler;
            mod_handler.visit_file_mut(st);
            let stt = quote!(#st);
            let mut new_file = fs::File::create(path).unwrap();
            new_file.write_all(stt.to_string().as_bytes()).unwrap();
            new_file.sync_data().unwrap();
        }
    }
}
