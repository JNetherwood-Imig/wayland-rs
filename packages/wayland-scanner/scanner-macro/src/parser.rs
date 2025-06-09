use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use change_case::pascal_case;
use serde::Deserialize;
use syn::Ident;

pub fn parse_protocol(path: &Path) -> Protocol {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    Protocol::from(quick_xml::de::from_reader::<_, RawProtocol>(reader).unwrap())
}

pub struct Protocol {
    pub name: String,
    pub copyright: Option<Copyright>,
    pub description: Option<Description>,
    pub interfaces: Vec<Interface>,
}

impl From<RawProtocol> for Protocol {
    fn from(value: RawProtocol) -> Self {
        Self {
            name: if value.name == "wayland" {
                "wl".to_string()
            } else {
                value.name
            },
            copyright: value.copyright.map(Copyright::from),
            description: value.description.map(Description::from),
            interfaces: value.interfaces.into_iter().map(Interface::from).collect(),
        }
    }
}

struct Copyright(String);

impl From<RawCopyright> for Copyright {
    fn from(value: RawCopyright) -> Self {
        Self(value.0.trim().to_string())
    }
}

pub struct Interface {
    pub name: String,
    pub type_name: String,
    pub max_version: u32,
    pub description: Option<Description>,
    pub requests: Vec<Request>,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
}

impl From<RawInterface> for Interface {
    fn from(value: RawInterface) -> Self {
        let name = value
            .name
            .strip_prefix("wl_")
            .unwrap_or(&value.name)
            .to_string();
        let type_name = pascal_case(&name);
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
            name,
            type_name,
            max_version: value.version.parse().unwrap(),
            description: value.description.map(Description::from),
            requests,
            events,
            enums,
        }
    }
}

pub enum RequestType {
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

pub struct Request {
    pub name: String,
    pub r#type: RequestType,
    pub since: u32,
    pub deprecated_since: Option<u32>,
    pub description: Option<Description>,
    pub args: Vec<Arg>,
}

impl From<RawRequest> for Request {
    fn from(value: RawRequest) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type.into(),
            since: value.since.map_or(1, |s| s.parse().unwrap()),
            deprecated_since: value.deprecated_since.map(|s| s.parse().unwrap()),
            description: value.description.map(Description::from),
            args: value.args.into_iter().map(Arg::from).collect(),
        }
    }
}

pub enum EventType {
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

pub struct Event {
    pub name: String,
    pub type_name: String,
    pub r#type: EventType,
    pub since: u32,
    pub deprecated_since: Option<u32>,
    pub description: Option<Description>,
    pub args: Vec<Arg>,
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        let name = value.name;
        let type_name = pascal_case(&name);
        Self {
            name,
            type_name,
            r#type: value.r#type.into(),
            since: value.since.map_or(1, |s| s.parse().unwrap()),
            deprecated_since: value.deprecated_since.map(|s| s.parse().unwrap()),
            description: value.description.map(Description::from),
            args: value.args.into_iter().map(Arg::from).collect(),
        }
    }
}

pub struct Enum {
    pub name: String,
    pub type_name: String,
    pub since: u32,
    pub description: Option<Description>,
    pub entries: Vec<Entry>,
}

impl From<RawEnum> for Enum {
    fn from(value: RawEnum) -> Self {
        let type_name = change_case::pascal_case(&value.name);
        Self {
            name: value.name,
            type_name,
            since: value.since.map_or(1, |s| s.parse().unwrap()),
            description: value.description.map(Description::from),
            entries: value.entries.into_iter().map(Entry::from).collect(),
        }
    }
}

pub struct Entry {
    pub name: String,
    pub valid_name: String, // FIXME chang_case crate does not correctly make pascal case when numbers are present
    pub value: u32,
    pub summary: Option<String>,
    pub since: u32,
    pub deprecated_since: Option<u32>,
    pub description: Option<Description>,
}

impl From<RawEntry> for Entry {
    fn from(value: RawEntry) -> Self {
        let mut valid_name = pascal_case(&value.name);
        if syn::parse_str::<Ident>(&value.name).is_err() {
            valid_name.insert(0, '_');
        }

        Self {
            name: value.name,
            valid_name,
            value: match value.value.strip_prefix("0x") {
                Some(hex) => u32::from_str_radix(hex, 16).unwrap(),
                _ => u32::from_str_radix(&value.value, 10).unwrap(),
            },
            summary: value.summary,
            since: value.since.map_or(1, |s| s.parse().unwrap()),
            deprecated_since: value.deprecated_since.map(|s| s.parse().unwrap()),
            description: value.description.map(Description::from),
        }
    }
}

pub enum ArgType {
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

pub struct Arg {
    pub name: String,
    pub r#type: ArgType,
    pub summary: Option<String>,
    pub nullable: bool,
    pub description: Option<Description>,
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

pub struct Description {
    pub summary: String,
    pub content: Option<String>,
}

impl From<RawDescription> for Description {
    fn from(value: RawDescription) -> Self {
        Self {
            summary: value.summary,
            content: value.content.map(|s| s.trim().to_string()),
        }
    }
}

#[derive(Deserialize)]
struct RawProtocol {
    #[serde(rename = "@name")]
    name: String,
    copyright: Option<RawCopyright>,
    description: Option<RawDescription>,
    #[serde(default, rename = "interface")]
    interfaces: Vec<RawInterface>,
}

#[derive(Deserialize)]
struct RawCopyright(String);

#[derive(Deserialize)]
struct RawInterface {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
    description: Option<RawDescription>,
    #[serde(rename = "$value")]
    elements: Vec<Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Element {
    Request(RawRequest),
    Event(RawEvent),
    Enum(RawEnum),
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
struct RawDescription {
    #[serde(rename = "@summary")]
    summary: String,
    #[serde(rename = "$text")]
    content: Option<String>,
}
