# mass-ip-pinger

> Now in C++!

A simple tool to ping a list of IP addresses, and check for interesting ports left open.

## Usage

```
sudo ./ipPinger --list-maker 16 google_ips output
```

> Using sudo is required to send ICMP packets on linux.

This will ping all IP addresses specified in the file `google_ips` (ip ranges attributed to google for crawlers and such), and output the results to `output`.

See the formatting used in `google_ips` to create your own list of IP addresses to ping.

You can also use an already existing list of IP addresses, you will have to run the program with `--use-unwrapped` instead of `--list-maker`. The program will then expect the IP addresses to be separated by a newline.

```
sudo ./ipPinger --use-unwrapped 16 ips
```

This option does not require the `output` argument. It will output the results to `testedIps` by default.

The program will create n*2 output files, where n is the number of threads used to ping (16, in the example). You can be generous with the number of threads, as the program will encounter many timeouts.

You can then use the `recombiner.py` script to combine the output files into one.

```
python3 recombiner.py output
```

This will create a file called `recombined.out` containing all the results. Only the IP addresses that responded to the ping will be kept.

## Output

The output file will contain the IP address, followed by a list of ports that were open. The ports are separated by a comma.

```
<IP address>;<host name>;[<open port>,<open port>,...]
```

## Building

```
cmake .
make
```

## System requirements

This program was tested on Debian 11. It should work on any linux distribution, and on Windows (with WSL).

It needs access to the raw socket, so it needs to be run as root especially on linux.

I've tested it with 16 threads on an Intel Core 2 Duo with 8GB of RAM, and it worked fine. It should work on any modern computer.

Be careful, there are no checks performed to see if the number of threads is coherent, you can easily crash your computer by using too many threads. Same goes for the number of IP addresses to ping. It will probably work with all the 4,2 billion IP addresses, but you'll need a few gigabytes of RAM.

## Disclaimer

This program is for educational purposes only. I am not responsible for any damage you may cause with it. Use at your own risk.

Be aware that you will ping a lot of IPs very quickly, and port scans will be performed. You could end up on a blacklist if you're not careful (or if running this program on a network you don't own).

Be careful and use your brain !

## License

GPLv3 (see [GPLv3](gnu.org/licenses/gpl-3.0.en.html))

nlouis56 2023