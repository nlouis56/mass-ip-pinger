# mass-ip-pinger

Python scripts to generate lists of "up" ips from standard ip attribution tables

> Because of the very nature of this project, you will very quickly ping a lot of ips. Please be considerate of your network and the networks you are pinging. This is not a tool to be used for malicious purposes.

## Specifications

- Python 3.11 (any version of python 3 should work if you replace the match case statement in `run.py`)
- Cross platform (tested on Windows and Linux)
- No external dependencies except the "requests" library

## How does it work?

The script `run.py` takes a file containing a list of ip ranges. The ranges can be in the following format:

- `1.1.1.1;255.255.255.255;4294967296`

Essentially, the script takes the first ip, the last ip, and the number of ips in the range. It then generates a list of ips from the first ip to the last ip.

- `<start_ip>;<end_ip>;<num_ips>`

Then, the big list is partitioned into n smaller lists, where n is the number of threads you want to use. Each thread then pings each ip in its list and writes the "up" and "down" ips to a file.

Every operation is using as many threads as possible and sensible. The script focuses on cpu usage and not on memory usage.

## How to use

1. Clone the repository
2. Install the dependency `requests` with `pip install requests`
3. Run the script with `python run.py`
> Note: All the filepaths are determined in the global variables at the top of the script. You can change them to your liking, or use the confirmation prompt to change them at runtime.

## Example

The file `google_ips` contains a list of ip ranges for Google. The script will generate a list of "up" ips and a list of "down" ips.

The file `french_ip_list` contains a list of ip ranges for France, retrieved from [ip2location.com](https://lite.ip2location.com/france-ip-address-ranges).

You just have to change the `INPUT_FILE` variable to the path of the file containing the ip ranges.

The script will then run, and generate a list of n files containing the results of the ping test for every ip. n corresponds to the number of threads you specified.

## TODO

- Find a way to restart the threads when they crash or get stuck (happens sometimes, i don't know why)
- Make the prints a little prettier
- Add a script to recombine the "up" and "down" lists into a single list
- Add a script to scan for open ports on the "up" ips

## License

This project is licensed under the GPL-3.0 License - see the [GPL-3.0 License](https://www.gnu.org/licenses/gpl-3.0.en.html) for details.