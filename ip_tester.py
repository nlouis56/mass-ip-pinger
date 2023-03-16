import concurrent.futures
import connection_tester
import time

ct = connection_tester.ConnectionTester()
TESTED_IPS = 0
TOTAL_IPS = 0
UP_IPS = 0
AVG_RQ_TIME = 0

def does_file_exist(filepath):
    try:
        with open(filepath, 'r') as f:
            return True
    except FileNotFoundError:
        return False

def print_manager():
    timeout_counter = 0
    if (TOTAL_IPS == 0 or TESTED_IPS == 0):
        time.sleep(1)
        timeout_counter += 1
        if timeout_counter > 10:
            print ("Timeout while waiting for the threads to start")
            return
    while TESTED_IPS < TOTAL_IPS:
        print (f"{TESTED_IPS} ips tested ({round((TESTED_IPS / TOTAL_IPS) * 100, 2)}%) - {UP_IPS} up addresses ({round((UP_IPS / TESTED_IPS) * 100, 2)}) - avg. request time: {round(AVG_RQ_TIME, 2)}ms")
        time.sleep(5)

def tester(filepath, threadID):
    global TESTED_IPS, TOTAL_IPS, UP_IPS, AVG_RQ_TIME
    with open(filepath, 'r') as f:
        for _ in enumerate(f):
            TOTAL_IPS += 1
    current_line = 0
    start_line = 0
    up_count = 0
    if does_file_exist(filepath+'.out'):
        print(f"Thread {threadID} - Output file already exists, resuming the completion...")
        with open(filepath+'.out', 'r') as outfile:
            for line in outfile:
                start_line += 1
                if line.split(';')[1] == 'UP':
                    up_count += 1
        print(f"Thread {threadID} - Resumed at {start_line} ips tested - - {up_count} up / {start_line} tested ({up_count/start_line*100}%))")
        return
    with open(filepath, 'r') as infile:
        for line in infile:
            if current_line < start_line:
                current_line += 1
                continue
            ip = line.strip()
            ctres, elapsed = ct.is_up(ip)
            TESTED_IPS += 1
            AVG_RQ_TIME = ((AVG_RQ_TIME * (TESTED_IPS - 1)) + elapsed) / TESTED_IPS
            if not ctres:
                out = f"{ip};DOWN\n"
            else:
                out = f"{ip};UP;{ctres[0]}\n"
                UP_IPS += 1
            with open(filepath+'.out' , 'a') as outfile:
                    outfile.write(out)
            current_line += 1

def start_threads(thread_count, base_filepath):
    with concurrent.futures.ThreadPoolExecutor(max_workers=thread_count + 2) as executor:
        results = []
        results.append(executor.submit(print_manager))
        for i in range(thread_count):
            results.append(executor.submit(tester, f'{base_filepath}.{i+1}', i+1))
        print(f"All {thread_count} threads started, waiting for results...")
        for future in concurrent.futures.as_completed(results):
            print(future.result())

def main():
    print("Starting standalone ip tester, please run 'python3 run.py' for the whole experience")
    start_threads(20, './separated_lists/iplist')

if __name__ == "__main__":
    main()