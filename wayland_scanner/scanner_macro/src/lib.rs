use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use client::GenClientTokens;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod client;
pub(crate) mod parser;
mod path;
pub(crate) mod server;

#[proc_macro]
pub fn generate_client_protocols(_input: TokenStream) -> TokenStream {
    let files = collect_protocol_files();

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

fn collect_protocol_files() -> Vec<PathBuf> {
    let dirs = get_paths();
    let mut paths = Vec::<PathBuf>::new();
    for dir in dirs {
        collect_protocols_helper(dir.as_path(), &mut paths);
    }
    paths
}

fn collect_protocols_helper(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("Failed to read dir") {
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

fn get_paths() -> Vec<PathBuf> {
    if let Ok(path) = env::var("WAYLAND_PROTOCOLS_PATH") {
        return path.split(':').map(PathBuf::from).collect();
    }
    if let Ok(path) = env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(path);
        if path.join("wayland").is_dir() && path.join("wayland-protocols").is_dir() {
            return vec![path.join("wayland"), path.join("wayland-protocols")];
        }
    }
    if let Ok(path) = env::var("XDG_DATA_DIRS") {
        if let Some(p) = path
            .split(':')
            .map(Path::new)
            .find(|base| base.join("wayland").is_dir() && base.join("wayland-protocols").is_dir())
            .map(Path::to_path_buf)
        {
            return vec![p.join("wayland"), p.join("wayland-protocols")];
        }
    }
    match fs::exists("/usr/share") {
        Ok(true) => vec![PathBuf::from("/usr/share")],
        _ => Vec::new(),
    }
}
