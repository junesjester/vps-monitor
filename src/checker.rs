use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

pub fn check_port(target: &str) -> bool {
    let addr: SocketAddr = match target.parse() {
        Ok(a) => a,
        Err(_) => return false,
    };

    TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok()
}