from concurrent.futures import ThreadPoolExecutor, as_completed
import time


TOTAL_LINES_PROCESSED = 0
TOTAL_LINES = 0
LINES_WRITTEN = 0

OUTPUT_PATH = ""


def increment_ip(ip):
    ip = ip.split('.')
    ip = [int(i) for i in ip]
    if ip[3] < 255:
        ip[3] += 1
    else:
        ip[3] = 0
        if ip[2] < 255:
            ip[2] += 1
        else:
            ip[2] = 0
            if ip[1] < 255:
                ip[1] += 1
            else:
                ip[1] = 0
                ip[0] += 1
    ip = [str(i) for i in ip]
    ip = '.'.join(ip)
    return ip


def is_ip_inferior(ip1, ip2):
    ip1 = ip1.split('.')
    ip2 = ip2.split('.')
    intip1 = [int(i) for i in ip1]
    intip2 = [int(i) for i in ip2]
    i = 3
    while i >= 0:
        if intip1[i] <= intip2[i]:
            pass
        elif intip1[i] > intip2[i]:
            return False
        i -= 1
    return True


def parse_current_line(line):
    line = line.strip()
    splt = line.split(';')
    start_ip = splt[0].strip()
    end_ip = splt[1].strip()
    try:
        qty = int(splt[2].strip())
    except ValueError:
        print("ValueError: {} is not a valid ip quantity".format(qty))
        qty = 0
    return start_ip, end_ip, qty


def generate_ips(start_ip, end_ip, qty):
    #print("Generating {} ips from {} to {}".format(qty, start_ip, end_ip))
    data = []
    current_ip = start_ip
    for _ in range(qty):
        if not is_ip_inferior(current_ip, end_ip):
            # print (">> END IP EXCEEDED : {} is not inferior to {}".format(current_ip, end_ip))
            break
        data.append(current_ip)
        current_ip = increment_ip(current_ip)
    #print("Generated {} ips".format(len(data)))
    return data


def print_progress():
    while TOTAL_LINES_PROCESSED < TOTAL_LINES:
        print(f"Total lines processed: {TOTAL_LINES_PROCESSED}/{TOTAL_LINES} ({round(TOTAL_LINES_PROCESSED / TOTAL_LINES * 100, 2)}%), {LINES_WRITTEN} lines written")
        time.sleep(1)
    return


def write_lines(future):
    global TOTAL_LINES_PROCESSED, LINES_WRITTEN, OUTPUT_PATH
    TOTAL_LINES_PROCESSED += 1
    lines = future.result()
    with open(OUTPUT_PATH, 'a') as outfile:
        try:
            if outfile.readline() != '':
                outfile.write('\n')
        except Exception:
            pass
        for line in lines:
            outfile.write(line + '\n')
        LINES_WRITTEN += len(lines)


def make_full_list(path, output_path='iplist'):
    global TOTAL_LINES_PROCESSED, TOTAL_LINES, LINES_WRITTEN, OUTPUT_PATH
    OUTPUT_PATH = output_path
    with open(path, 'r') as infile:
        for _ in enumerate(infile):
            TOTAL_LINES += 1
    with ThreadPoolExecutor(max_workers=None) as executor:
        futures = []
        print(f"Starting workers to make the full list in {output_path}...")
        futures.append(executor.submit(print_progress))
        with open(path, 'r') as infile:
            for line in infile:
                parsed_line = parse_current_line(line)
                future = executor.submit(generate_ips, parsed_line[0], parsed_line[1], parsed_line[2])
                future.add_done_callback(write_lines)
                futures.append(future)
    return LINES_WRITTEN


if __name__ == "__main__":
    print ("Starting ip_maker.py")
    make_full_list("ip_fr")
    print ("\n\n - - - - - - DONE - - - - - - ")
