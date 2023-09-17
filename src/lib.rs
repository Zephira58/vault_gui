use serde;
use serde_yaml;
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::{self},
    io::Write,
    net::{IpAddr, TcpStream},
    path::Path,
    time::Duration,
};

pub fn string_to_bool (config: String) -> bool {
    let enabled = config.trim();
    let truth_value: bool = match enabled {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        _ => false  // Or whatever appropriate default value or error.
    };

    return truth_value
}


pub async fn is_server_alive(ip: IpAddr, port: u16, timeout_secs: u64) -> bool {
    if let Ok(_) = TcpStream::connect_timeout(&(ip, port).into(), Duration::from_secs(timeout_secs))
    {
        true
    } else {
        false
    }
}

pub fn increment(mut num: i32) -> i32 {
    num = num + 1;
    return num
}

pub fn validate_ip_address(ip_address: &str) -> bool {
    // Regular expression pattern for matching IP addresses
    let pattern =
        r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(ip_address)
}

pub fn config_manager() /*-> BTreeMap<&str, &str>*/ {
    let dir = Path::new("./");

    let config_check = fs::OpenOptions::new()
        .read(true)
        .open(dir.to_str().unwrap().to_owned() + "config.yaml");

    match config_check  {
        Err(_) => {
            fs::create_dir_all(dir.clone()).unwrap();
            let mut config_file = fs::OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(dir.to_str().unwrap().to_owned() + "config.yaml")
                .expect("create failed");

            let mut yaml_data = BTreeMap::new();
            yaml_data.insert("01-enabled", "false");
            yaml_data.insert("02-ip", "");
            yaml_data.insert("03-port", "3306");
            yaml_data.insert("04-username", "");
            yaml_data.insert("05-password", "");
            

            let yaml = serde_yaml::to_string(&yaml_data);
            println!("{:?}", yaml);

            let _ = config_file.write_all(b"#Enter your MySQL information below for caching\n");
            let _ = config_file.write_all(yaml.as_bytes());
            //pre-inputs values if none are already present

            return yaml_data
        },
        Ok(_) => {
            fs::create_dir_all(dir.clone()).unwrap();
            let mut config_file = fs::OpenOptions::new()
                .write(false)
                .read(true)
                .create(false)
                .open(dir.to_str().unwrap().to_owned() + "config.yaml")
                .expect("create failed");

            let mut yaml_data = serde_yaml::from_str(&config_file);
        }
    }
}
