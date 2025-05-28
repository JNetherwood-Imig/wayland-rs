type Copyright = String;

struct Protocol {
    name: String,
    copyright: Option<Copyright>,
    description: Option<Description>,
    interfaces: Vec<Interface>,
}

struct ParsedProtocol {
    name: String,
    copyright: Option<Copyright>,
    description: Option<Description>,
    interfaces: Vec<ParsedInterface>,
}

struct Interface {
    name: String,
    version: String,
    description: Option<Description>,
    requests: Vec<Request>,
    events: Vec<Event>,
    enums: Vec<Enum>,
}

struct ParsedInterface {
    name: String,
    version: u32,
    description: Option<Description>,
    requests: Vec<ParsedRequest>,
    events: Vec<ParsedEvent>,
    enums: Vec<ParsedEnum>,
}

struct Request {
    name: String,
    r#type: Option<String>,
    since: Option<String>,
    deprecated_since: Option<String>,
    description: Option<Description>,
    args: Vec<Arg>,
}

enum RequestType {
    Default,
    Destructor,
}

struct ParsedRequest {
    name: String,
    r#type: RequestType,
    since: u32,
    deprecated_since: u32,
    args: Vec<Arg>,
}

struct Event {
    name: String,
    r#type: Option<String>,
    since: Option<String>,
    deprecated_since: Option<String>,
    description: Option<Description>,
    args: Vec<Arg>,
}

enum EventType {
    Default,
    Destructor,
}

struct ParsedEvent {
    name: String,
    r#type: EventType,
    since: u32,
    deprecated_since: u32,
    args: Vec<Arg>,
}

struct Enum {
    name: String,
    since: Option<String>,
    bitfield: Option<String>,
    description: Option<Description>,
    entries: Vec<Entry>,
}

struct ParsedEnum {
    name: String,
    since: u32,
    bitfield: bool,
    description: Option<Description>,
    entries: Vec<ParsedEntry>,
}

struct Entry {
    name: String,
    value: String,
    summary: Option<String>,
    since: Option<String>,
    deprecated_since: Option<String>,
    description: Option<Description>,
}

struct ParsedEntry {
    name: String,
    value: u32,
    summary: Option<String>,
    since: u32,
    deprecated_since: u32,
    description: Option<String>,
}

struct Arg {
    name: String,
    r#type: String,
    summary: Option<String>,
    interface: Option<String>,
    allow_null: Option<String>,
    r#enum: Option<String>,
    description: Option<Description>,
}

enum ArgType {
    Int,
    Uint,
    Fixed,
    String,
    Array,
    NewId(String),
    Fd,
    GenericNewId,
    Enum(String),
}

struct ParsedArg {
    name: String,
    r#type: ArgType,
    summary: Option<String>,
}

struct Description {
    summary: String,
    content: Option<String>,
}
