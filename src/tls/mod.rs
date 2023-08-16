#![allow(non_snake_case)]

use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype, SslConnector, SslVerifyMode};
use crate::{
    debug
};
use std::{
    net::{TcpStream, Shutdown},
    sync::{Arc},
    thread
};
use spin::mutex::{Mutex};
pub mod cert;
pub mod utils;

fn setup_cert(host: String) -> Result<Arc<SslAcceptor>, ()>{
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    let pwd = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    acceptor.set_private_key_file(pwd.clone() + "/tls/cert/key.pem", SslFiletype::PEM).unwrap();
    let Some(certificate) = cert::setup_tls_cert(host.clone()) else { return Err(()); };
    let Ok(x509_cert) = cert::decodeX509(certificate.clone()) else { return Err(()); };
    acceptor.set_certificate(&x509_cert).unwrap();
    acceptor.check_private_key().unwrap();
    Ok(Arc::new(acceptor.build()))
}

fn new_tls_connection(host: String, port: String) -> Result<SslStream<TcpStream>, ()> {
    let connector = {
        let Ok(mut builder) = SslConnector::builder(SslMethod::tls()) else { return Err(()); };
        builder.set_verify(SslVerifyMode::NONE);
        builder.build()
    };
    let Ok(stream) = TcpStream::connect(host.clone() + ":" + &port.to_string()) else { return Err(()); };
    let Ok(stream) = connector.connect(&host.clone(), stream) else { return Err(()); };
    Ok(stream)
}

fn print_packet(src: Arc<Mutex<SslStream<TcpStream>>>, dst: Arc<Mutex<SslStream<TcpStream>>>, buf: Vec<u8>) {
    unsafe {
        src.force_unlock();
        dst.force_unlock();
    }
    let mut src = src.lock();
    let src = src.get_mut();
    let mut dst = dst.lock();
    let dst = dst.get_mut();
    debug::rx_tx_packet(src.try_clone(), dst.try_clone(), buf);
}

fn peer_to_peer(src: Arc<Mutex<SslStream<TcpStream>>>, dst: Arc<Mutex<SslStream<TcpStream>>>) {
    let mut buf: [u8; 65536] = [0; 65536];
    loop {
        let len = src.lock().ssl_read(&mut buf).unwrap_or(0);
        if len == 0 {
            let _ = src.lock().shutdown();
            return;
        }
        print_packet(src.clone(), dst.clone(), buf[0..len].to_vec());
        unsafe {
            dst.force_unlock();
        }
        if dst.lock().ssl_write(&buf[0..len]).unwrap_or(0) == 0 {
            let _ = src.lock().shutdown();
            return;
        }
    }
}

fn begin_connection(src: SslStream<TcpStream>, dst: SslStream<TcpStream>){
    let source = Arc::new(Mutex::<SslStream<TcpStream>>::new(src));
    let dest = Arc::new(Mutex::<SslStream<TcpStream>>::new(dst));
    
    /* client -> server */
    let (arc_src, arc_dst) = (source.clone(), dest.clone());
    thread::spawn(move || {
        peer_to_peer(arc_src, arc_dst);
    });

    /* server -> client */
    let (arc_src, arc_dst) = (source.clone(), dest.clone());
    thread::spawn(move || {
        peer_to_peer(arc_dst, arc_src);
    });

}

pub fn handleTlsConnection<T>(client: Result<TcpStream, T>, host: String) {
    let Ok(client) = client else { return; };
    debug::new_connection(true, client.try_clone());
    let Ok(mut stream) = new_tls_connection(host.clone(), "443".to_string()) else { return; };
    let Ok(tls_cert) = setup_cert(host.clone()) else { return; };
    let Ok(client) = tls_cert.accept(client.try_clone().unwrap()) else { 
        debug::PROXY_DEBUG!(0b00001000, "error: tls handshake");
        let _ = client.shutdown(Shutdown::Both);
        let _ = stream.shutdown();
        return;
    };
    debug::PROXY_DEBUG!(0b00000010, "tls connection established");
    begin_connection(client, stream);
}
