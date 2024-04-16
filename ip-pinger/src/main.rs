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

    /// The timeout for each ping in milliseconds (Not implemented yet)
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

fn threader(mut chunks: Vec<Vec<Ipv4Addr>>, thread_qty: usize, timeout: u64, ports: Vec<u16>) {
    let mut threads = Vec::new();
    while let Some(thread_data) = chunks.pop() {
        if threads.len() < thread_qty {
            threads.push(std::thread::spawn(move || runpings(thread_data, timeout)));
        } else {
            let mut idx: usize = 0;
            while !threads[idx].is_finished() {
                idx += 1;
                if idx >= threads.len() {
                    idx = 0;
                }
            }
            let finished_thread = threads.swap_remove(idx);
            let finishedprops = finished_thread.join().unwrap();
            let (up, _) = stats::sort_up_from_down(&finishedprops);
            saver::save_to_file(&up, "up_ips.csv");
            for ip in up {
                let open_ports = pings::scanports_mt(ip.ip, &ports, timeout);
                if open_ports.len() > 0 {
                    saver::save_ports_to_file(&ip, &open_ports, "open_ports.csv");
                }
            }
        }
    }
    // Wait for remaining threads to finish
    for thread in threads {
        let _ = thread.join().unwrap();
    }
}

fn singlethread(ips: Vec<Ipv4Addr>, timeout: u64, ports: Vec<u16>) {
    let props = runpings(ips, timeout);
    let (up, _) = stats::sort_up_from_down(&props);
    saver::save_to_file(&up, "up_ips.csv");
    for ip in up {
        let open_ports = pings::scanports_st(ip.ip, &ports, timeout);
        if open_ports.len() > 0 {
            saver::save_ports_to_file(&ip, &open_ports, "open_ports.csv");
        }
    }
}

fn launcher(args: Args) {
    let mut ips = parser::parse_ips(&args.file);
    let mut rng = thread_rng();
    println!("Shuffling IPs...");
    ips.shuffle(&mut rng);
    println!("Shuffled !");
    let mut chunks: Vec<Vec<Ipv4Addr>> = Vec::new();
    if args.threads == 1 {
        println!("Running in single thread mode");
    } else {
        println!("Running in multi-thread mode");
        chunks = ips.chunks_exact(args.bite)
            .map(|chunk| chunk.to_vec())
            .collect();
        println!("Separated IPs into {} chunks", chunks.len());
    }
    let ports: Vec<u16>;
    if args.full {
        println!("Running a full port scan");
        ports = (1..=65535).collect();
    } else {
        println!("Running a partial port scan");
        ports = parser::parse_ports(&args.ports);
    }
    if chunks.len() == 0 {
        singlethread(ips, args.timeout, ports);
    } else {
        threader(chunks, args.threads, args.timeout, ports);
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
    if !args.full && !std::path::Path::new(&args.ports).exists() {
        println!("File {} does not exist", args.ports);
        std::process::exit(1);
    }
    println!("Will be running on {} with {} threads", args.file, args.threads);
    launcher(args);
}
