from time import sleep
from concurrent.futures import ThreadPoolExecutor, as_completed
from itertools import islice


TOTAL_LINES_PROCESSED = 0
TOTAL_LINES = 0
FILES_WRITTEN = 0
TOTAL_FILES = 0
DONE = False

def print_manager():
    global TOTAL_LINES_PROCESSED, TOTAL_LINES, FILES_WRITTEN
    timeout_counter = 0
    if (TOTAL_LINES_PROCESSED == 0 or TOTAL_LINES == 0):
        sleep(1)
        timeout_counter += 1
        if timeout_counter > 10:
            print ("Timeout while waiting for the threads to start")
            return
    while not DONE:
        print (f'{FILES_WRITTEN} files have been created, {TOTAL_LINES_PROCESSED} lines processed ({round(TOTAL_LINES_PROCESSED / TOTAL_LINES * 100, 2)}%)')
        sleep(5)
    print(f"{FILES_WRITTEN} files out of {TOTAL_FILES} have been created, done splitting.")
    return

def split_file_by_lines(filepath, quantity, output_folder):
    global TOTAL_LINES_PROCESSED, TOTAL_LINES, FILES_WRITTEN, TOTAL_FILES, DONE
    # Calculate the number of lines in each output file
    TOTAL_FILES = quantity
    with open(filepath, 'r') as infile:
        for _ in enumerate(infile):
            TOTAL_LINES += 1
    lines_per_file = TOTAL_LINES // quantity
    print (f'Will be writing {lines_per_file} lines per file ({quantity} files in total)')
    # Create the output files
    for i in range(quantity):
        start_idx = i * lines_per_file
        end_idx = (i + 1) * lines_per_file if i < quantity - 1 else TOTAL_LINES
        try:
            with open(f'./{output_folder}/{filepath}.{i+1}', 'w') as outfile:
                with open(filepath, 'r') as infile:
                    lines = list(islice(infile, start_idx, end_idx))
                    TOTAL_LINES_PROCESSED += len(lines)
                outfile.writelines(lines)
                FILES_WRITTEN += 1
        except Exception as e:
            print (f"Error while writing file {i+1}: {e}")
    DONE = True
    return


def start_splitting(filepath, quantity, output_folder):
    futures = []
    with ThreadPoolExecutor(max_workers=None) as executor:
        splitter = executor.submit(split_file_by_lines, filepath, quantity, output_folder)
        printmgr = executor.submit(print_manager)
        futures.append(splitter)
        futures.append(printmgr)
    print (f"Wrote all lines ({TOTAL_LINES_PROCESSED}/{TOTAL_LINES}) into {FILES_WRITTEN} files")
    return


if __name__ == "__main__":
    split_file_by_lines('iplist', 20, 'cut_list')
