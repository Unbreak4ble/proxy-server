mod http_utils;
mod proxy;
mod connection;
mod constants;
mod debug;
mod utils;
mod tls;

#[tokio::main]
async fn main() {
    println!("welcome to {}", constants::PROXY_NAME);
    let _ = proxy::start().await;
}
