extern crate proc_macro;
use proc_macro::TokenStream;
use serde::Deserialize;
use syn::{LitStr, parse_macro_input};

use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
    str,
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
    let p: RawProtocol = quick_xml::de::from_reader(reader).unwrap();
    println!("{:#?}", p);
}

#[derive(Debug, Deserialize)]
struct RawProtocol {
    #[serde(rename = "@name")]
    name: String,
    copyright: Option<RawCopyright>,
    description: Option<RawDescription>,
    #[serde(default, rename = "interface")]
    interfaces: Vec<RawInterface>,
}

#[derive(Debug, Deserialize)]
struct RawCopyright(String);

#[derive(Debug, Deserialize)]
struct RawInterface {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
    description: Option<RawDescription>,
    #[serde(rename = "$value")]
    elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Element {
    Request(RawRequest),
    Event(RawEvent),
    Enum(RawEnum),
}

#[derive(Debug, Deserialize)]
struct RawRequest {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<String>,
    description: Option<RawDescription>,
    #[serde(default, rename = "arg")]
    args: Vec<RawArg>,
}

#[derive(Debug, Deserialize)]
struct RawEvent {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<String>,
    description: Option<RawDescription>,
    #[serde(default, rename = "arg")]
    args: Vec<RawArg>,
}

#[derive(Debug, Deserialize)]
struct RawEnum {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@bitfield")]
    bitfield: Option<String>,
    description: Option<RawDescription>,
    #[serde(default, rename = "entry")]
    entries: Vec<RawEntry>,
}

#[derive(Debug, Deserialize)]
struct RawEntry {
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
    description: Option<RawDescription>,
}

#[derive(Debug, Deserialize)]
struct RawArg {
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
    description: Option<RawDescription>,
}

#[derive(Debug, Deserialize)]
struct RawDescription {
    #[serde(rename = "@summary")]
    summary: String,
    content: Option<String>,
}

struct Protocol {
    name: String,                     // snake_case
    copyright: Option<Copyright>, // leading and trailing whitespace removed, split at /n, with each line prefixed by //
    description: Option<Description>, // leading and trailing whitespace removed, split at /n, with each line prefixed by ///
    interfaces: Vec<Interface>,       // one or more
}

struct Copyright(String);

struct Interface {
    name: String,      // snake_case
    type_name: String, // PascalCase
    max_version: u32,  // parsed from version
    description: Option<Description>,
    requests: Vec<Request>,
    events: Vec<Event>,
    enums: Vec<Enum>,
}

enum RequestType {
    Default,
    Destructor,
}

struct Request {
    name: String, // snake_case
    r#type: RequestType,
    since: u32,
    deprecated_since: Option<u32>,
    description: Option<Description>,
    args: Vec<Arg>,
}

enum EventType {
    Default,
    Destructor,
}

struct Event {
    name: String, // snake_case
    r#type: EventType,
    since: u32,
    deprecated_since: Option<u32>,
    description: Option<Description>,
    args: Vec<Arg>,
}

enum EnumType {
    Int,
    UInt,
    Bitfield,
}

struct Enum {
    name: String,      // snake_case
    type_name: String, // PascalCase
    since: u32,
    r#type: EnumType,
    description: Option<Description>,
    entries: Vec<Entry>,
}

struct Entry {
    name: String,       // snake_case
    valid_name: String, // PascalCase, invalid identifiers fixed
    value: u32,
    summary: Option<String>,
    since: u32,
    deprecated_since: Option<u32>,
    description: Option<Description>,
}

impl From<RawEntry> for Entry {
    fn from(value: RawEntry) -> Self {}
}

enum ArgType {
    Int,
    UInt,
    Enum(String),
    Fixed,
    String,
    Object(String),
    NewId(String),
    UnspecifiedNewId,
    Array,
    Fd,
}

impl From<(String, Option<String>, Option<String>)> for ArgType {
    fn from(value: (String, Option<String>, Option<String>)) -> Self {
        let t = value.0;
        let interface = value.1;
        let en = value.2;

        match t.as_str() {
            "int" => en.map_or(Self::Int, Self::Enum),
            "uint" => en.map_or(Self::UInt, Self::Enum),
            "fixed" => Self::Fixed,
            "string" => Self::String,
            "object" => interface.map_or(Self::UInt, Self::Object),
            "new_id" => interface.map_or(Self::UnspecifiedNewId, Self::NewId),
            "array" => Self::Array,
            "fd" => Self::Fd,
            _ => unreachable!(),
        }
    }
}

struct Arg {
    name: String,
    r#type: ArgType,
    summary: Option<String>,
    nullable: bool,
    description: Option<Description>,
}

impl From<RawArg> for Arg {
    fn from(value: RawArg) -> Self {
        Self {
            name: value.name,
            r#type: (value.r#type, value.interface, value.r#enum).into(),
            summary: value.summary,
            nullable: value
                .allow_null
                .as_deref()
                .map_or(false, |s| s.eq_ignore_ascii_case("true")),
            description: value.description.map(Description::from),
        }
    }
}

struct Description {
    summary: String,
    content: Option<String>,
}

impl From<RawDescription> for Description {
    fn from(value: RawDescription) -> Self {
        let content = if let Some(content) = value.content {
            Some(content.trim().to_string())
        } else {
            None
        };

        Self {
            summary: value.summary,
            content,
        }
    }
}
