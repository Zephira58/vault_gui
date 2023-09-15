use config::Config;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self},
    io::Write,
    net::{IpAddr, TcpStream},
    path::Path,
    time::Duration,
};

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

pub fn config_manager() -> HashMap<String, String> {
    let dir = Path::new("./");

    let config_check = fs::OpenOptions::new()
        .read(true)
        .open(dir.to_str().unwrap().to_owned() + "config.toml");

    match config_check {
        Err(_) => {
            fs::create_dir_all(dir.clone()).unwrap();
            let mut config_file = fs::OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(dir.to_str().unwrap().to_owned() + "config.toml")
                .expect("create failed");

            let _ = config_file.write_all(b"#Enter your MySQL information below for caching\nip = ''\nport = '3306'\nusername = ''\npassword = ''"); //pre-inputs values if none are already present
        }
        Ok(_) => {

        }
    }

    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    //Sets the variable "hi" to a hashmaped version of the config.toml file.
    
    settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()

    //Reads the hashmap with the key of "test" to get the value
    //match hi.get("test") {
    //    Some(hi) => println!("{}", hi),
    //    _ => println!("error"),
    //}
}
