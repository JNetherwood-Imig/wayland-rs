extern crate proc_macro;
use change_case::pascal_case;
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
    let p = quick_xml::de::from_reader::<BufReader<File>, RawProtocol>(reader).unwrap();
    println!("{:#?}", Protocol::from(p));
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

#[derive(Debug)]
struct Protocol {
    name: String,                     // snake_case
    copyright: Option<Copyright>, // leading and trailing whitespace removed, split at /n, with each line prefixed by //
    description: Option<Description>, // leading and trailing whitespace removed, split at /n, with each line prefixed by ///
    interfaces: Vec<Interface>,       // one or more
}

impl From<RawProtocol> for Protocol {
    fn from(value: RawProtocol) -> Self {
        Self {
            name: value.name,
            copyright: value.copyright.map(Copyright::from),
            description: value.description.map(Description::from),
            interfaces: value.interfaces.into_iter().map(Interface::from).collect(),
        }
    }
}

#[derive(Debug)]
struct Copyright(String);

impl From<RawCopyright> for Copyright {
    fn from(value: RawCopyright) -> Self {
        Self(value.0.trim().to_string())
    }
}

#[derive(Debug)]
struct Interface {
    name: String,      // snake_case
    type_name: String, // PascalCase
    max_version: u32,  // parsed from version
    description: Option<Description>,
    requests: Vec<Request>,
    events: Vec<Event>,
    enums: Vec<Enum>,
}

impl From<RawInterface> for Interface {
    fn from(value: RawInterface) -> Self {
        let type_name = pascal_case(&value.name);
        let mut requests = Vec::<Request>::new();
        let mut events = Vec::<Event>::new();
        let mut enums = Vec::<Enum>::new();
        for elem in value.elements {
            match elem {
                Element::Request(req) => requests.push(req.into()),
                Element::Event(ev) => events.push(ev.into()),
                Element::Enum(en) => enums.push(en.into()),
            };
        }
        Self {
            name: value.name,
            type_name,
            // max_version: value.version.parse().unwrap(),
            max_version: 1,
            description: value.description.map(Description::from),
            requests,
            events,
            enums,
        }
    }
}

#[derive(Debug)]
enum RequestType {
    Default,
    Destructor,
}

impl From<Option<String>> for RequestType {
    fn from(value: Option<String>) -> Self {
        match value.as_deref() {
            Some("destructor") => Self::Destructor,
            _ => Self::Default,
        }
    }
}

#[derive(Debug)]
struct Request {
    name: String, // snake_case
    r#type: RequestType,
    since: u32,
    deprecated_since: Option<u32>,
    description: Option<Description>,
    args: Vec<Arg>,
}

impl From<RawRequest> for Request {
    fn from(value: RawRequest) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type.into(),
            // since: value.since.map_or(1, |s| s.parse().unwrap()),
            since: 1,
            // deprecated_since: value.deprecated_since.map(|s| s.parse().unwrap()),
            deprecated_since: Some(0),
            description: value.description.map(Description::from),
            args: value.args.into_iter().map(Arg::from).collect(),
        }
    }
}

#[derive(Debug)]
enum EventType {
    Default,
    Destructor,
}

impl From<Option<String>> for EventType {
    fn from(value: Option<String>) -> Self {
        match value.as_deref() {
            Some("destructor") => Self::Destructor,
            _ => Self::Default,
        }
    }
}

#[derive(Debug)]
struct Event {
    name: String, // snake_case
    r#type: EventType,
    since: u32,
    deprecated_since: Option<u32>,
    description: Option<Description>,
    args: Vec<Arg>,
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type.into(),
            // since: value.since.map_or(1, |s| s.parse().unwrap()),
            since: 1,
            // deprecated_since: value.deprecated_since.map(|s| s.parse().unwrap()),
            deprecated_since: Some(0),
            description: value.description.map(Description::from),
            args: value.args.into_iter().map(Arg::from).collect(),
        }
    }
}

#[derive(Debug)]
struct Enum {
    name: String,      // snake_case
    type_name: String, // PascalCase
    since: u32,
    description: Option<Description>,
    entries: Vec<Entry>,
}

impl From<RawEnum> for Enum {
    fn from(value: RawEnum) -> Self {
        let type_name = change_case::pascal_case(&value.name);
        Self {
            name: value.name,
            type_name,
            // since: value.since.map_or(1, |s| s.parse().unwrap()),
            since: 1,
            description: value.description.map(Description::from),
            entries: value.entries.into_iter().map(Entry::from).collect(),
        }
    }
}

#[derive(Debug)]
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
    fn from(value: RawEntry) -> Self {
        let is_valid_ident = syn::parse_str::<syn::Ident>(value.name.as_str()).is_ok();
        println!("{} is valid identifier: {}", value.name, is_valid_ident);
        // let numeric_value = value.name.parse().unwrap();
        let numeric_value = 0u32;
        let valid_name = change_case::pascal_case(&value.name);
        Self {
            name: value.name,
            valid_name,
            value: numeric_value,
            summary: value.summary,
            // since: value.since.map_or(1, |s| s.parse().unwrap()),
            since: 1,
            // deprecated_since: value.deprecated_since.map(|s| s.parse().unwrap()),
            deprecated_since: Some(0),
            description: value.description.map(Description::from),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Description {
    summary: String,
    content: Option<String>,
}

impl From<RawDescription> for Description {
    fn from(value: RawDescription) -> Self {
        Self {
            summary: value.summary,
            content: value.content.map(|s| s.trim().to_string()),
        }
    }
}
