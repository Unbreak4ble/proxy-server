#![allow(dead_code)]

use std::process::Command;
use openssl::x509::X509;
use crate::tls::utils;
use std::fs;

pub fn setup_tls_cert(host: String) -> Option<String> {
    let pwd = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/tls/cert";
    let CACert = format!("{pwd}/rootca.crt");
    let CAKey = format!("{pwd}/rootca.key");
    let certKey = format!("{pwd}/key.pem");
    let host_type = if utils::isIp(host.clone()) { "IP" } else { "DNS" };
    let Ok(cmdOut) = Command::new("sh").arg("-c").arg(format!("openssl x509 -req -extfile <(echo \"subjectAltName={host_type}:{host}\") -CA {CACert} -CAkey {CAKey} -days 360 -sha256 -CAcreateserial -in <(openssl req -new -sha256 -key {certKey} -subj \"/C=US/ST=North Carolina/O=ORG/OU=ORG_UNIT/CN=localhost\" -reqexts SAN -config <(cat /etc/ssl/openssl.cnf <(printf \"\n[SAN]\nsubjectAltName=DNS:localhost\")))")).output() else { return None; };
    Some(String::from_utf8_lossy(&cmdOut.stdout).to_string())
}

pub fn get_pkcs8_key() -> Option<String> {
    let pwd = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/tls/cert";
    let Ok(content) = fs::read_to_string(pwd + "/private.key") else { return None; };
    Some(content)
}

pub fn get_pkey_pem() -> Option<String> {
    let pwd = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/tls/cert";
    let Ok(content) = fs::read_to_string(pwd + "/key.pem") else { return None; };
    Some(content)
}

pub fn get_p12() -> Option<Vec<u8>> {
    let pwd = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/tls/cert";
    let Ok(content) = fs::read(pwd + "/cert.p12") else { return None; };
    Some(content)
}
pub fn decodeX509(content: String) -> Result<X509, ()> {
    let Ok(cert) = X509::from_pem(&content.as_bytes()) else { return Err(()) };
    Ok(cert)
}
