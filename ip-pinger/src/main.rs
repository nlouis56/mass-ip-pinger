use clap::Parser;
use std::net::Ipv4Addr;
use rand::{seq::SliceRandom, thread_rng};

use crate::pings::runpings;
pub mod parser;
pub mod pings;
pub mod stats;
pub mod saver;

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
    #[arg(long, default_value = "false")]
    full: bool,

    /// The number of threads to use
    #[arg(short, long, default_value = "16")]
    threads: usize,

    /// The number of slices to make from the list
    #[arg(short, long, default_value = "256")]
    slices: usize,

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
    println!("Enter the number of slices to make from the list: ");
    let mut slices = String::new();
    std::io::stdin().read_line(&mut slices).unwrap();
    let slices: usize = slices.trim().parse().unwrap();
    println!("Enter the timeout for each ping in milliseconds: ");
    let mut timeout = String::new();
    std::io::stdin().read_line(&mut timeout).unwrap();
    let timeout: u64 = timeout.trim().parse().unwrap();
    return Args {
        file,
        ports,
        full,
        threads,
        slices,
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

fn threader(chunks: Vec<Vec<Ipv4Addr>>, thread_qty: usize, timeout: u64) {
    for chunk in chunks {
        let mut threads = Vec::new();
        let threads_data = make_ip_chunks(chunk, thread_qty);
        for data in threads_data {
            let thread = std::thread::spawn(move || runpings(data, timeout));
            threads.push(thread);
        }
        for finished in threads {
            let chunk_addresses = finished.join().unwrap();
            stats::show_stats(&chunk_addresses);
            let (up, _) = stats::sort_up_from_down(&chunk_addresses);
            saver::save_to_file(&up, "up_ips.csv");
        }
    }
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
    let mut rng = thread_rng();
    let mut ips = parser::main(&args.file);
    println!("Shuffling IPs...");
    ips.shuffle(&mut rng);
    println!("Shuffled !");
    // split the ips into threads chunks
    let chunks = make_ip_chunks(ips, args.slices);
    println!("Separated IPs into {} chunks", chunks.len());
    // ping the ips
    threader(chunks, args.threads, args.timeout);
}
