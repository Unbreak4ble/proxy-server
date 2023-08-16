#![allow(dead_code)]
/*
 * bit combinations:
 * 0b00000000 = none
 * 0b10000000 = display new clients connected
 * 0b01000000 = display from/to connected addresses
 * 0b00100000 = display from/to connected domain names
 * 0b00010000 = display rx/tx raw packet data
 * 0b00001000 = display connection errors
 * 0b00000100 = display internal errors
 * 0b00000010 = display proxy logs
*/
pub const DEBUG: u8 = 0b00010000;

/*
 * used to decrypt tls connection
*/
pub const TLS: bool = false;

pub const IP_BIND: &str = "0.0.0.0";
pub const PORT_BIND: u16 = 1080;

pub const PROXY_NAME: &str = "Xproxy";
pub const PROXY_VERSION: &str = "1.0";

pub const HTTP_VERSION: &str = "1.1";

