extern crate proc_macro;
use proc_macro::TokenStream;
use serde::Deserialize;
use syn::{LitStr, parse_macro_input};

use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

#[proc_macro]
pub fn generate_client_protocols(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let mut files = Vec::<PathBuf>::new();
    collect_protocols_helper(&PathBuf::from(input), &mut files);

    for file in files.iter() {
        generate_code_for_protocol(file);
    }

    TokenStream::new()
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

fn generate_code_for_protocol(path: &Path) {
    println!("{}\n", path.to_str().unwrap());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let p: Protocol = quick_xml::de::from_reader(reader).unwrap();
    println!("{:#?}", p);
}

#[derive(Debug, Deserialize)]
struct Protocol {
    #[serde(rename = "@name")]
    name: String,
    copyright: Option<Copyright>,
    description: Option<Description>,
    #[serde(default, rename = "interface")]
    interfaces: Vec<Interface>,
}

#[derive(Debug, Deserialize)]
struct Copyright(String);

#[derive(Debug, Deserialize)]
struct Interface {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
    description: Option<Description>,
    #[serde(rename = "$value")]
    elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Element {
    Request(Request),
    Event(Event),
    Enum(Enum),
}

#[derive(Debug, Deserialize)]
struct Request {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<String>,
    description: Option<Description>,
    #[serde(default, rename = "arg")]
    args: Vec<Arg>,
}

#[derive(Debug, Deserialize)]
struct Event {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<String>,
    description: Option<Description>,
    #[serde(default, rename = "arg")]
    args: Vec<Arg>,
}
#[derive(Debug, Deserialize)]
struct Enum {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@bitfield")]
    bitfield: Option<String>,
    description: Option<Description>,
    #[serde(default, rename = "entry")]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@summary")]
    summary: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Arg {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@summary")]
    summary: Option<String>,
    #[serde(rename = "@interface")]
    interface: Option<String>,
    #[serde(rename = "@allow-null")]
    allow_null: Option<String>,
    #[serde(rename = "@enum")]
    r#enum: Option<String>,
    description: Option<Description>,
}

#[derive(Debug, Deserialize)]
struct Description {
    #[serde(rename = "@summary")]
    summary: String,
    content: Option<String>,
}
