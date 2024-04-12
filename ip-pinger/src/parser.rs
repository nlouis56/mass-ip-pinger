use std::net::Ipv4Addr;
use std::vec::Vec;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct IpRange {
    pub start_ip: Ipv4Addr,
    pub end_ip: Ipv4Addr,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_csv(filepath: &String) -> Vec<IpRange> {
    let mut contents: Vec<IpRange> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let line: Vec<&str> = line.split(";").collect();
            let start_ip: Ipv4Addr = line[0].parse().unwrap();
            let end_ip: Ipv4Addr = line[1].parse().unwrap();
            contents.push(IpRange {
                start_ip,
                end_ip,
            });
        }
    }
    contents
}

fn is_ip_valid(ip: Ipv4Addr) -> bool {
    let ip = ip.to_string();
    let ip: Vec<&str> = ip.split(".").collect();
    if ip.len() != 4 {
        return false;
    }
    for octet in ip {
        match octet.parse::<u8>() {
            Ok(_) => {}
            Err(_) => return false,
        }
    }
    true
}

fn add_to_ip(ip: Ipv4Addr, n: u32) -> Ipv4Addr {
    let ip: u32 = u32::from(ip);
    let result: u32 = ip.wrapping_add(n);
    Ipv4Addr::from(result)
}

fn unwrap_ips(record: IpRange) -> Vec<Ipv4Addr> {
    let mut ips: Vec<Ipv4Addr> = Vec::new();
    let mut i: u32 = 0;
    loop {
        let ip: Ipv4Addr = add_to_ip(record.start_ip, i);
        if ip > record.end_ip { break; }
        ips.push(ip);
        i += 1;
    }
    ips
}

pub fn main(filepath: &String) -> Vec<Ipv4Addr> {
    let records: Vec<IpRange> = read_csv(filepath);
    let mut valid_records: Vec<IpRange> = Vec::new();
    for record in records {
        if is_ip_valid(record.start_ip) && is_ip_valid(record.end_ip) {
            valid_records.push(record);
        }
    }
    println!("{} valid records found", valid_records.len());
    let mut ips: Vec<Ipv4Addr> = Vec::new();
    for record in valid_records {
        let mut record_ips: Vec<Ipv4Addr> = unwrap_ips(record);
        ips.append(&mut record_ips);
    }
    println!("{} IPs found after unwrapping", ips.len());
    ips
}
