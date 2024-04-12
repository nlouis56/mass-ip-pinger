use clap::Parser;
use std::net::Ipv4Addr;

use crate::pings::{runpings, IpProperties};
pub mod parser;
pub mod pings;

#[derive(Parser)]
#[command(name = "ip-pinger", version, about)]
struct Args {
    /// The file to read IP addresses from
    #[arg(short, long, required = false)]
    file: String,

    /// The file containing a list of ports to scan
    #[arg(short, long, required = false)]
    ports: String,

    /// Do a full port scan (1-65535)
    #[arg(long)]
    full: bool,

    /// The number of threads to use
    #[arg(short, long, default_value = "16")]
    threads: usize,

    /// The timeout for each ping in milliseconds
    #[arg(short = 'b', long, default_value = "500")]
    timeout: u64,
}

fn cli() -> Args {
    // get user input
    println!("Current path: {}", std::env::current_dir().unwrap().display());
    println!("Enter the file to read IP addresses from: ");
    let mut file = String::new();
    std::io::stdin().read_line(&mut file).unwrap();
    file = file.trim().to_string();
    println!("Do a full port scan (1-65535) [y/n]: ");
    let mut full = String::new();
    std::io::stdin().read_line(&mut full).unwrap();
    full = full.trim().to_string();
    let full = if full == "y" { true } else { false };
    let mut ports = String::new();
    if !full {
        println!("Enter the file containing a list of ports to scan: ");
        std::io::stdin().read_line(&mut ports).unwrap();
        ports = ports.trim().to_string();
    }
    println!("Enter the number of threads to use: ");
    let mut threads = String::new();
    std::io::stdin().read_line(&mut threads).unwrap();
    let threads: usize = threads.trim().parse().unwrap();
    println!("Enter the timeout for each ping in milliseconds: ");
    let mut timeout = String::new();
    std::io::stdin().read_line(&mut timeout).unwrap();
    let timeout: u64 = timeout.trim().parse().unwrap();
    return Args {
        file,
        ports,
        full,
        threads,
        timeout,
    };
}

fn make_ip_chunks(ips: Vec<Ipv4Addr>, threads: usize) -> Vec<Vec<Ipv4Addr>> {
    let mut chunks: Vec<Vec<Ipv4Addr>> = Vec::new();
    let chunk_size = ips.len() / threads;
    let mut i = 0;
    for _ in 0..threads {
        let mut chunk: Vec<Ipv4Addr> = Vec::new();
        for _ in 0..chunk_size {
            chunk.push(ips[i]);
            i += 1;
        }
        chunks.push(chunk.to_vec());
    }
    chunks
}

fn main() {
    let args: Args;
    println!("received {} arguments", std::env::args().len());
    if std::env::args().len() == 1 {
        println!("No arguments passed, using CLI");
        args = cli();
    } else {
        // arguments passed, parse them
        args = Args::parse();
    }
    // check if the file exists
    if !std::path::Path::new(&args.file).exists() {
        println!("File {} does not exist", args.file);
        std::process::exit(1);
    }
    println!("Will be running on {} with {} threads", args.file, args.threads);
    let ips = parser::main(&args.file);
    // split the ips into threads chunks
    let chunks = make_ip_chunks(ips, args.threads);
    println!("Separated IPs into {} chunks", chunks.len());
    // ping the ips
    let mut addresses: Vec<IpProperties> = Vec::new();
    for chunk in chunks {
        let propchunk = runpings(chunk, args.timeout);
        println!("Got {} properties from pinger", propchunk.len());
        addresses.extend(propchunk);
    }
}
