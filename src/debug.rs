#![allow(dead_code, unused_variables)]

use std::net::TcpStream;
use crate::http_utils;
use crate::utils;

macro_rules! DEBUG {
    ($mode: literal) => {
        if crate::constants::DEBUG & $mode != $mode {
            return;
        }
    }
}


macro_rules! PROXY_DEBUG {
    ($mode: literal, $($content: tt)*) => {
        if (crate::constants::DEBUG & $mode == $mode) || ($mode & 0xFF == 0xFF){
            print!("[{}]: ", crate::utils::get_time());
            println!($($content)*);
        }
    }
}

pub(crate) use PROXY_DEBUG;

pub fn connection_to_from_ip<T>(src: Result<TcpStream, T>, dest: Result<TcpStream, T>) {
    DEBUG!(0b01000000);
    let Ok(src) = src else { return; };
    let Ok(dest) = dest else { return; };
    PROXY_DEBUG!(0xFF, "{:?} -> {:?}", src.peer_addr().unwrap(), dest.peer_addr().unwrap());
}

pub fn connection_to_from_http<T>(client: Result<TcpStream, T>, packet: http_utils::Payload) {
    DEBUG!(0b00100000);
    let Ok(client) = client else { return; };
    let client_ip = client.peer_addr().unwrap();
    let http_method = packet.http_header.method;
    let http_utils::Host::domain(http_host) = packet.http_header.host else { return; };
    let port = packet.http_header.port;
    PROXY_DEBUG!(0xFF, "{:?} -> {:?} {}:{}",
        client_ip,
        http_method,
        http_host,
        port);
    
}

pub fn new_connection<T>(tls: bool, client: Result<TcpStream, T>) {
    DEBUG!(0b10000000);
    let Ok(client) = client else { return; };
    if tls {
        PROXY_DEBUG!(0xFF, "new tls client {:?}", client.peer_addr().unwrap());
    }else {
        PROXY_DEBUG!(0xFF, "new client {:?}", client.peer_addr().unwrap());
    }
}

pub fn rx_tx_packet<T>(src: Result<TcpStream, T>, dest: Result<TcpStream, T>, data: Vec<u8>) {
    DEBUG!(0b00010000);
    let Ok(src) = src else { return; };
    let Ok(dest) = dest else { return; };
    let mut content = format!("{:?} -> {:?}\n", src.peer_addr().unwrap(), dest.peer_addr().unwrap());
    content += &utils::hexdump(data);
    println!("\n{content}");
}
