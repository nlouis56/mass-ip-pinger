import concurrent.futures
import scripts.connection_tester as connection_tester
import time

ct = connection_tester.ConnectionTester()
TESTED_IPS = 0
TOTAL_IPS = 0
UP_IPS = 0
AVG_RQ_TIME = 0


THREADPOOL = None

class TesterThread:
    def __init__(self, threadID) -> None:
        self.threadID = threadID
        self.filepath = ""
        self.start_line = 0
        self.up_count = 0
        self.current_line = 0
        self.timeout_counter = 0
        self.up_ips = 0
        self.avg_rq_time = 0
        self.tested_ips = 0
        self.total_ips = 0
        self.start_time = 0
        self.end_time = 0
        self.future = None
        self.thread_running = False

    def start(self, threadID, filepath):
        self.threadID = threadID
        self.filepath = filepath
        self.start_time = time.time()
        self.thread_running = True


    def do_tests(self):
        with open(self.filepath, 'r') as f:
            for _ in enumerate(f):
                self.total_ips += 1

    def is_frozen(self):
        if self.thread_running:
            return False
        return True


def does_file_exist(filepath):
    try:
        with open(filepath, 'r') as f:
            return True
    except FileNotFoundError:
        return False


def print_manager(testers):
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


def find_starting_line(filepath):
    start_line, up_count = 0, 0
    if not does_file_exist(filepath):
        return start_line, up_count
    with open(filepath+'.out', 'r') as outfile:
            for line in outfile:
                start_line += 1
                if line.split(';')[1] == 'UP':
                    up_count += 1
    return start_line, up_count


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
        start_line, up_count = find_starting_line(filepath+'.out')
        print(f"Thread {threadID} - Resumed at {start_line} ips tested - - {up_count} up / {start_line} tested ({up_count/start_line*100}%))")
    with open(filepath, 'r') as infile:
        for current_line, line in infile:
            if current_line < start_line:
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


def start_threads(thread_count, base_filepath):
    global THREADPOOL
    testers = [TesterThread(id) for id in range(thread_count)]
    with concurrent.futures.ThreadPoolExecutor(max_workers=thread_count) as executor:
        THREADPOOL = executor
        executor.submit(print_manager, testers)
        for tester in testers:
            pass
        print_manager()
        for thread in THREADPOOL:
            thread.join()

def main():
    print("Starting standalone ip tester, please run 'python3.11 run.py' for the whole experience")
    thread_count = int(input("How many threads do you want to use? "))
    base_filepath = input("What is the base filepath? ")
    start_threads(thread_count, base_filepath)

if __name__ == "__main__":
    main()