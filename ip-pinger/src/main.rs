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

    /// The number of ips on which one thread should run
    /// Influences the frequency of the stats output and the file saving
    #[arg(long, default_value = "4096")]
    bite: usize,

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
    println!("Enter the size of the bite for each thread: ");
    let mut bite = String::new();
    std::io::stdin().read_line(&mut bite).unwrap();
    let bite: usize = bite.trim().parse().unwrap();
    println!("Enter the timeout for each ping in milliseconds: ");
    let mut timeout = String::new();
    std::io::stdin().read_line(&mut timeout).unwrap();
    let timeout: u64 = timeout.trim().parse().unwrap();
    return Args {
        file,
        ports,
        full,
        threads,
        bite,
        timeout,
    };
}

fn threader(mut chunks: Vec<Vec<Ipv4Addr>>, thread_qty: usize, timeout: u64) {
    let mut threads = Vec::new();
    while let Some(thread_data) = chunks.pop() {
        if threads.len() < thread_qty {
            threads.push(std::thread::spawn(move || runpings(thread_data, timeout)));
        } else {
            // Wait for a thread to finish before launching a new one
            let thread = threads.remove(0);
            let results = thread.join().unwrap(); // Handle thread panic gracefully if needed
            let (up, _) = stats::sort_up_from_down(&results);
            stats::show_stats(&results);
            saver::save_to_file(&up, "up_ips.csv");
            threads.push(std::thread::spawn(move || runpings(thread_data, timeout)));
        }
    }
    // Wait for remaining threads to finish
    for thread in threads {
        let _ = thread.join().unwrap(); // Handle thread panic gracefully if needed
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
    let chunks: Vec<Vec<Ipv4Addr>> = ips.chunks_exact(args.bite)
        .map(|chunk| chunk.to_vec())
        .collect();
    println!("Separated IPs into {} chunks", chunks.len());
    // ping the ips
    threader(chunks, args.threads, args.timeout);
}
