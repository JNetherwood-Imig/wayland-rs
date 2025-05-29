use std::fs;
use std::path::{Path, PathBuf};

use client::GenClientTokens;
use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

pub(crate) mod client;
pub(crate) mod parser;
pub(crate) mod server;

#[proc_macro]
pub fn generate_client_protocols(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let mut files = Vec::<PathBuf>::new();
    collect_protocols_helper(&PathBuf::from(input), &mut files);

    let generated = files
        .iter()
        .map(|f| parser::parse_protocol(f).to_tokens())
        .collect::<Vec<proc_macro2::TokenStream>>();

    let output = quote! {
        #( #generated )*
    };

    println!("{}", output);

    output.into()
}

fn collect_protocols_helper(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_protocols_helper(&path, files);
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext.eq_ignore_ascii_case("xml") {
                    files.push(path);
                }
            }
        }
    }
}
