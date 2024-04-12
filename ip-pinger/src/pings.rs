use std::time::Duration;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use dns_lookup::getnameinfo;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;

pub struct IpProperties {
    pub ip: Ipv4Addr,
    pub up: bool,
    pub rtt: Duration,
    pub hostname: String,
}

fn gethostname(ip: Ipv4Addr) -> String {
    let socketaddr = SocketAddr::new(ip.into(), 0);
    let flags = 0;

    match getnameinfo(&socketaddr, flags) {
        Ok(hostname) => {return hostname.0},
        Err(_) => {return String::from("Unknown")},
    };
}

pub fn runpings(addresses: Vec<Ipv4Addr>, timeout: u64) -> Vec<IpProperties> {
    let mut properties: Vec<IpProperties> = Vec::new();
    let (pinger, results) = match Pinger::new(Some(timeout), None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };
    println!("Pinger created with timeout of {} ms", timeout);
    for ip in &addresses {
        pinger.add_ipaddr(&ip.to_string());
    }
    println!("Pinging {} addresses", addresses.len());
    pinger.ping_once();
    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    let v4addr = Ipv4Addr::from_str(&addr.to_string()).unwrap();
                    let props = IpProperties {
                        ip: v4addr,
                        up: false,
                        rtt: Duration::from_millis(0),
                        hostname: String::from("Unknown"),
                    };
                    println!("{} is down", v4addr);
                    properties.push(props);
                }
                Receive { addr, rtt } => {
                    let v4addr = Ipv4Addr::from_str(&addr.to_string()).unwrap();
                    let props = IpProperties {
                        ip: v4addr,
                        up: true,
                        rtt,
                        hostname: gethostname(v4addr),
                    };
                    println!("{} is up with latency of {} ms. Host is {}", v4addr, props.rtt.as_millis() / 2, props.hostname);
                    properties.push(props);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
        if properties.len() == addresses.len() {
            return properties;
        }
    }
}
