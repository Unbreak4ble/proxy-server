#![allow(dead_code, non_camel_case_types, unused_variables)]

use regex::Regex;
use crate::constants;

#[derive(Debug)]
pub enum HttpError {
    InvalidHttp,
    InvalidHttpHeader,
    InvalidHeaders,
    Other(String),
    Null,
}

#[derive(Clone, Debug)]
pub enum Method {
    CONNECT,
    GET,
    POST,
    NULL
}

impl Default for Method {
    fn default() -> Self {
        Self::NULL
    }
}

#[derive(Clone, Debug)]
pub enum Host {
    domain(String),
    ipv4(String),
    ipv6(String),
    NULL
}

impl Default for Host {
    fn default() -> Self {
        Self::NULL
    }
}

#[derive(Clone, Debug)]
pub enum Proto {
    HTTP,
    HTTPS,
    WS,
    WSS,
    NULL
}

impl Default for Proto {
    fn default() -> Self {
        Self::NULL
    }
}

#[derive(Clone, Debug, Default)]
pub struct Property(String, String);

#[derive(Clone, Debug, Default)]
pub struct Header {
    pub method: Method,
    pub protocol: Proto,
    pub host: Host,
    pub port: u16,
    pub path: String,
    pub version: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Payload {
    pub http_header: Header,
    pub headers: Vec<Property>,
    pub data: String,
}

pub fn get_header_value(headers: Vec<Property>, name: &str) -> Option<String> {
    for Property(property, value) in headers {
        if property.to_lowercase() == name.to_lowercase() {
            return Some(value);
        }
    }
    None
}

fn enum_host(host: &mut Host, buffer: String) {
    let path_reg = Regex::new("(\\/.*)$").unwrap();
    let proto_reg = Regex::new("^(.*:\\/\\/)").unwrap();
    let port_reg = Regex::new("(:\\d+)$").unwrap();
    let domain_path = proto_reg.replace_all(&buffer, "");
    let domain = path_reg.replace_all(&domain_path, "");
    let domain = port_reg.replace_all(&domain, "");
    *host = Host::domain(domain.to_string());
}

fn decode_port(header: &mut Header, buffer: String) {
    let path_reg = Regex::new("(\\/.*)$").unwrap();
    let proto_reg = Regex::new("^(.*:\\/\\/)").unwrap();
    let port_reg = Regex::new("([^:.]\\d+)$").unwrap();
    let domain_path = proto_reg.replace_all(&buffer, "");
    let domain = path_reg.replace_all(&domain_path, "");
    let port = port_reg.find(&domain);
    if let Some(port) = port {
        header.port = port.as_str().parse::<u16>().unwrap_or(0);
    }else{
        header.port = match header.protocol {
            Proto::HTTPS => 443,
            _ => 80
        }
    }
}

fn decode_path(path: &mut String, buffer: String) {
    let reg = Regex::new("[^.A-z]{1,}(([^.]*)$)").unwrap();
    let regport = Regex::new(":\\d+").unwrap();
    let Some(pat) = reg.captures(&buffer) else { return; };
    let Some(pat) = pat.get(0) else { return; };
    let pat = regport.replace_all(pat.as_str(), "");
    *path = pat.to_string();
}

fn decode_protocol(protocol: &mut Proto, buffer: String) {
    let reg = Regex::new("^([^:]*)").unwrap();
    let Some(proto) = reg.captures(&buffer) else { return; };
    let Some(proto) = proto.get(0) else { return; };
    let proto = proto.as_str();
    *protocol = match proto {
        "https" => Proto::HTTPS,
        "http" => Proto::HTTP,
        "ws" => Proto::WS,
        "wss" => Proto::WSS,
        _ => Proto::NULL,
    };
}

fn enum_method(method: &mut Method, string: String) {
    *method = match string.as_str() {
        "POST" => Method::POST,
        "GET" => Method::GET,
        "CONNECT" => Method::CONNECT,
        _ => Method::NULL
    };
}

fn decode_version(version: &mut u8, string: String) {
    *version = 0x11; //1.1
}

fn decode_header(buffer: String) -> Result<Header, HttpError> {
    let mut header: Header = Default::default();
    let strings = buffer.split(" ");
    let mut i=0;
    for string in strings {
        let string = string.to_string();
        match i {
            0 => enum_method(&mut header.method, string),
            1 => {
                decode_protocol(&mut header.protocol, string.clone());
                enum_host(&mut header.host, string.clone());
                decode_port(&mut header, string.clone());
                decode_path(&mut header.path, string.clone());
            },
            2 => decode_version(&mut header.version, string),
            _ => { return Err(HttpError::InvalidHttpHeader); },
        };
        i+=1;
    }
    Ok(header)
}

pub fn decode(buffer: &[u8]) -> Result<Payload, HttpError> {
    let mut payload: Payload = Default::default();
    let buffer = String::from_utf8_lossy(buffer);
    let reg = Regex::new("[^\\s].*[^\\r]").unwrap();
    let mut step=0;
    for line in buffer.split("\n") {
        match step {
            0 => {
                let header = decode_header(line.to_string());
                if let Ok(header) = header {
                    payload.http_header = header;
                }else if let Err(error) = header {
                    return Err(error);
                }
                step+=1;
            }
            1 => {
                if line == "\r" {
                    step+=1;
                    continue;
                }
                let header: Vec<String> = line.split(":").map(|x| x.to_string()).collect();
                if header.len() > 1 {
                    let Some(value) = reg.captures(&header[1]) else { continue; };
                    let Some(value) = value.get(0) else { continue; };
                    let value = value.as_str();
                    payload.headers.push(Property(header[0].clone(), String::from(value)));
                }
            }
            2 => {
                payload.data += (line.to_owned()+"\n").as_str();
            }
            _ => {}
        };
    }
    Ok(payload)
}

pub fn make_connection_response(status: u16, established: bool) -> String {
    let mut packet = String::from("HTTP/1.1 ".to_owned() + &status.to_string() + " " + if established { "Connection established" } else { "Connection failed" } + "\r\n");
    packet += &("Proxy-Agent: ".to_owned() + constants::PROXY_NAME + "/" + constants::PROXY_VERSION + "\r\n");
    packet + "\r\n"
}
