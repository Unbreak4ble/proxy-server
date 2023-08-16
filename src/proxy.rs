#![allow(unused_imports)]

use std::{
    io::{Write, Read},
    net::{TcpListener, TcpStream},
    thread
};
use crate::{
    http_utils,
    connection,
    debug,
    tls,
    constants
};
use tokio::{
    runtime::Handle,
    task
};

fn handle_client<T>(client: Result<TcpStream, T>) {
    let Ok(mut client) = client else { return; };
    let mut buff: [u8; 65536] = [0; 65536];
    let sz = client.read(&mut buff).unwrap_or(0);
    let Ok(packet) = http_utils::decode(&buff[0..sz]) else { return; };
    let http_utils::Host::domain(url) = packet.http_header.host.clone() else { return; };
    debug::connection_to_from_http(client.try_clone(), packet.clone());
    match packet.http_header.method {
        http_utils::Method::CONNECT => {
            let _ = client.write(http_utils::make_connection_response(200, true).as_bytes());
           if constants::TLS {
                tls::handleTlsConnection(client.try_clone(), url);
            }else{
                let Ok(stream) = connection::new_connection(url.to_string(), packet.http_header.port.to_string()) else { return; };
                connection::handle_connection(client.try_clone(), stream.try_clone());
            }
       },
        http_utils::Method::NULL => { return; },
        _ => {
            let _ = connection::iterate_connection(client.try_clone(), url.to_string(), packet.http_header.port.to_string(), buff[0..sz].to_vec());
        }
    }
}

pub async fn start() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(constants::IP_BIND.to_owned() + ":" + &constants::PORT_BIND.to_string())?;
    debug::PROXY_DEBUG!(0b00000010, "proxy running at port {}", constants::PORT_BIND);
    for client in listener.incoming() {
        let Ok(client) = client else { continue; };
        debug::new_connection(false, client.try_clone());
        thread::spawn(move || {
            handle_client(client.try_clone());
        });
    }
    Ok(())
}
