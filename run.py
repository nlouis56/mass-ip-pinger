import scripts.ip_maker as ip_maker
import scripts.list_partitioner as list_partitioner
import scripts.ip_tester as ip_tester
import os

THREADS = 16
EXECUTING_PATH = os.path.dirname(os.path.realpath(__file__))
IP_TABLE_NAME = "french_ip_partial"
FULL_LIST_NAME = "french_ip_partial_unwrapped"
SEPARATED_LISTS_FOLDER = "cut_list"

if __name__ == "__main__":
    print (f"Starting... {THREADS} threads\n")
    lines_written = ip_maker.make_full_list(os.path.join(EXECUTING_PATH, IP_TABLE_NAME), output_path=FULL_LIST_NAME)
    print (f"\nFull list has been created, {lines_written} lines written")
    print (f"Partitioning the list into {THREADS} files...\n")
    list_partitioner.start_splitting(FULL_LIST_NAME, THREADS, SEPARATED_LISTS_FOLDER)
    print ("\nThe list has been partitioned, starting the threads...\n")
    ip_tester.start_threads(THREADS, os.path.join(EXECUTING_PATH, SEPARATED_LISTS_FOLDER))
