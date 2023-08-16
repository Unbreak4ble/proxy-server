#![allow(dead_code)]

use std::net::{Shutdown, TcpStream};
use std::io::{Write, Read};
use std::thread;
use crate::debug;

#[derive(Debug)]
pub enum TcpError {
    ConnectionFailed,
    Refused,
    Other(String),
    Null,
}

fn peer_to_peer<T>(src: Result<TcpStream, T>, dst: Result<TcpStream, T>) {
    let Ok(mut src) = src else {return;};
    let Ok(mut dst) = dst else { return; };
    let mut buf: [u8; 65536] = [0; 65536];
    loop {
        let sz = src.read(&mut buf).unwrap_or(0);
        if sz == 0 {
            let _ = dst.shutdown(Shutdown::Both);
            return;
        }
        debug::rx_tx_packet(src.try_clone(), dst.try_clone(), buf[0..sz].to_vec());
        if dst.write(&buf[0..sz]).unwrap_or(0) == 0 {
            let _ = src.shutdown(Shutdown::Both);
            return;
        }
    }
}

pub fn handle_connection<T>(client: Result<TcpStream, T>, stream: Result<TcpStream, T>) {
    let Ok(client) = client else { return; };
    let Ok(stream) = stream else { return; };
    debug::connection_to_from_ip(client.try_clone(), stream.try_clone());
    
    let (src, dst) = (client.try_clone(), stream.try_clone());
    thread::spawn(move || {
        peer_to_peer(src, dst);
    });

    let (src, dst) = (client.try_clone(), stream.try_clone());
    thread::spawn(move || {
        peer_to_peer(dst, src);
    });
}

pub fn iterate_connection<T>(client: Result<TcpStream, T>, dest: String, port: String, packet: Vec<u8>) -> Result<(), TcpError> {
    let Ok(mut stream) = TcpStream::connect(dest+":"+&port) else { return Err(TcpError::ConnectionFailed); };
    let Ok(client) = client else {return Err(TcpError::ConnectionFailed);};
    match stream.write(&packet) {
        Err(..) => { return Err(TcpError::Refused); },
        Ok(size) => {
            if size == 0 {
                let _ = stream.shutdown(Shutdown::Both);
                return Ok(());
            }
        }
    };
    handle_connection(client.try_clone(), stream.try_clone());
    Ok(())
}

pub fn new_connection(dest: String, port: String) -> Result<TcpStream, TcpError> {
    //let (dest, port) = if constants::TLS { (String::from("0.0.0.0"), constants::TLS_PORT.to_string()) } else { (dest, port) };
    let Ok(stream) = TcpStream::connect(dest+":"+&port) else { return Err(TcpError::ConnectionFailed); };
    Ok(stream)
}
