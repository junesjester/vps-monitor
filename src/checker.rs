use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

pub fn check_port(target: &str) -> bool {
    let addr: SocketAddr = match target.parse() {
        Ok(a) => a,
        Err(_) => return false,
    };

    TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok()
}

pub async fn check_http(target: &str) -> bool {
    match reqwest::get(target).await {
        Ok(response) => response.status().is_success(),
        Err(_) => {
            println!("Could not make http GET request");
            false
        },
    }
}