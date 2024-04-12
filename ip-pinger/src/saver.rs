use std::io::Write;
use std::fs::OpenOptions;
use crate::pings::IpProperties;

pub fn save_to_file(properties: &Vec<IpProperties>, filename: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    let mut data = String::new();
    for prop in properties {
        data.push_str(&format!("{},{},{},{}\n", prop.ip, prop.up, prop.rtt.as_millis(), prop.hostname));
    }
    match file.write_all(data.as_bytes()) {
        Ok(_) => println!("Data saved to {}", filename),
        Err(e) => panic!("Error writing to file: {}", e),
    };
}
