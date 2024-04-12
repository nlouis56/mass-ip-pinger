# mass-ip-pinger

> Recoding the thing in rust !

For the other versions, see the branches.

A simple tool to ping a list of IP addresses, and check for interesting ports left open.

## Usage

```
cargo run -- -f google_ips.csv -p ports.txt
```

or

```
ip-pinger[.exe] -f google_ips.csv -p ports.txt
```

This will ping all IP addresses specified in the file `google_ips.csv` (ip ranges attributed to google for crawlers and such) and check for open ports specified in the file `ports.txt`.

The files are in the CSV format, with no headers, for example:

```csv
0.0.0.0;255.255.255.255
```

This will ping all the ip addresses of the internet (not really, but you get the idea).

Some websites provide lists of ip addresses containing the ranges and also the quantity of addresses in the range. Here, the quantity is not taken into account, only the start and end addresses are used.

The ports file is a simple list of ports, for example:

```txt
80
443
```

This will check if the ports 80 and 443 are open on the target ip addresses (if the target is reachable).

## Output

It doesn't output anything yet, the goal is to stuff the results in some kind of database for easy querying.

## Building

```
cargo build --release
```

(you will need cargo and rust installed, see [rustup](https://rustup.rs/))

## Dependencies

- [clap](https://crates.io/crates/clap) (for command line arguments)
- [fastping-rs]() (to ping the addresses)
- [dns-lookup](https://crates.io/crates/dns-lookup) (to get the hostname)

## System requirements

This program was tested on Debian 11. It should work on pretty much any Unix system as long as you have a working rust installation.

> Note: The program is not compatible with Windows, as it uses `fastping-rs` to ping the addresses, which is not compatible with the windows sockets.

## Disclaimer

This program is for educational purposes only. I am not responsible for any damage you may cause with it. Use at your own risk.

Be aware that you will ping a lot of IPs very quickly, and port scans will be performed. You could end up on a blacklist if you're not careful (or if you run this program on a network you don't own).

Be careful and use your brain !

## License

GPLv3 (see [GPLv3](gnu.org/licenses/gpl-3.0.en.html))

nlouis56 2023
