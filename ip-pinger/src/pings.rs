use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use dns_lookup::getnameinfo;

#[derive(Clone)]
pub struct IpProperties {
    pub ip: Ipv4Addr,
    pub up: bool,
    pub rtt: Duration,
    pub hostname: String,
}

impl IpProperties {
    pub fn new(ip: Ipv4Addr, up: bool, rtt: Duration, hostname: String) -> IpProperties {
        IpProperties {
            ip,
            up,
            rtt,
            hostname,
        }
    }

    pub fn clone(&self) -> IpProperties {
        IpProperties {
            ip: self.ip,
            up: self.up,
            rtt: self.rtt,
            hostname: self.hostname.clone(),
        }
    }
}

fn gethostname(ip: IpAddr) -> String {
    let socketaddr = SocketAddr::new(ip, 0);
    let flags = 0;

    match getnameinfo(&socketaddr, flags) {
        Ok(hostname) => {return hostname.0},
        Err(_) => {return String::from("Unknown")},
    };
}

fn scansingleport(ip: Ipv4Addr, port: u16, timeout: u64) -> bool {
    let socketaddr = SocketAddr::new(IpAddr::V4(ip), port);
    match std::net::TcpStream::connect_timeout(&socketaddr, Duration::from_millis(timeout)) {
        Ok(_) => {return true},
        Err(_) => {return false},
    };
}

pub fn scanports_st(ip: Ipv4Addr, ports: &Vec<u16>, timeout: u64) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();
    for &port in ports {
        let open = scansingleport(ip, port, timeout);
        if open {
            open_ports.push(port);
        }
    }
    return open_ports;
}

pub fn scanports_mt(ip: Ipv4Addr, ports: &Vec<u16>, timeout: u64) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();
    let mut threads = Vec::new();
    for &port in ports {
        let ip = ip.clone();
        threads.push(std::thread::spawn(move || {
            return scansingleport(ip, port, timeout);
        }));
    }
    for (idx, thread) in threads.into_iter().enumerate() {
        let thread_result: bool = thread.join().unwrap();
        if thread_result {
            println!("Port {} is open on {}", ports[idx], ip.to_string());
            open_ports.push(ports[idx]);
        }
    }
    return open_ports;
}

pub fn runpings(addresses: Vec<Ipv4Addr>, _: u64 /* timeout, unused */ ) -> Vec<IpProperties> {
    let mut properties: Vec<IpProperties> = Vec::new();
    for ip in addresses {
        let up: bool;
        let stopwatch = std::time::Instant::now();
        let hostname = gethostname(IpAddr::V4(ip));
        let rtt = stopwatch.elapsed();
        if hostname == "Unknown" || hostname == ip.to_string() {
            up = false;
        } else {
            up = true;
        }
        properties.push(IpProperties::new(ip, up, rtt, hostname));
    }
    return properties;
}
