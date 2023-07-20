use regex::Regex;
use std::net::{IpAddr, TcpStream};
use std::time::Duration;

pub async fn is_server_alive(ip: IpAddr, port: u16, timeout_secs: u64) -> bool {
    if let Ok(_) = TcpStream::connect_timeout(&(ip, port).into(), Duration::from_secs(timeout_secs))
    {
        true
    } else {
        false
    }
}

pub fn validate_ip_address(ip_address: &str) -> bool {
    // Regular expression pattern for matching IP addresses
    let pattern =
        r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(ip_address)
}
