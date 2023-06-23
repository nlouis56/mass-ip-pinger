# mass-ip-pinger

> Now in C++!

A simple tool to ping a list of IP addresses, and check for interesting ports left open.

## Usage

```
sudo ./ipPinger --unwrap google_ips --run-tests 16
```

> Using sudo is required to send ICMP packets on linux.

This will ping all IP addresses specified in the file `google_ips` (ip ranges attributed to google for crawlers and such), and output the results to `testedIps`.

See the formatting used in `google_ips` to create your own list of IP addresses to ping.

You can also use an already existing list of IP addresses, you will have to run the program with `--use-unwrapped` instead of `--unwrap`. The program will then expect the IP addresses to be separated by a newline.

```
sudo ./ipPinger --use-unwrapped ips --run-tests 16
```

As the previous command, it will output the results to `testedIps` by default.

The program will create n output files, where n is the number of threads used to ping (16, in the example). You can be generous with the number of threads, as the program will encounter many waiting periods while testing the ips (the tasks are not cpu-bound, so you can reduce the compute time by using more threads).

You can then use the `recombiner.py` script to combine the output files into one.

```
python3 recombiner.py
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

(you will need cmake and a c++ compiler)

## System requirements

This program was tested on Debian 11. It should work on any linux distribution, and on Windows (I don't know if WSL allows access to the sockets).

It needs access to the raw socket, so it needs to be run as root especially on linux.

I've tested it with 16 threads on an Intel Core 2 Duo with 8GB of RAM (2 cores, 2 threads), and it worked fine. It should work on any modern computer.

Be careful, there are no checks performed to see if the number of threads is coherent, you can easily crash your computer by using too many threads. Same goes for the number of IP addresses to ping. It will probably work with all the 4,2 billion IP addresses, but you'll need a few gigabytes of RAM. The program loads the IP addresses in memory, so the input file should not be bigger than the size of one of your ram sticks. The program performs a quick check to see if the input file is too big, but I've not tested the thing to see if it works.

## Disclaimer

This program is for educational purposes only. I am not responsible for any damage you may cause with it. Use at your own risk.

Be aware that you will ping a lot of IPs very quickly, and port scans will be performed. You could end up on a blacklist if you're not careful (or if you run this program on a network you don't own).

Be careful and use your brain !

## License

GPLv3 (see [GPLv3](gnu.org/licenses/gpl-3.0.en.html))

nlouis56 2023