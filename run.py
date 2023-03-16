import ip_maker
import list_partitioner
import ip_tester

THREADS = 1024
IP_TABLE_PATH = "./ip_fr_test"
FULL_LIST_PATH = "iplist_fr"
SEPARATED_LISTS_FOLDER = "cut_list"

if __name__ == "__main__":
    print (f"Starting... {THREADS} threads")
    lines_written = ip_maker.make_full_list(IP_TABLE_PATH, output_path=FULL_LIST_PATH)
    print (f"Full list has been created, {lines_written} lines written")
    print (f"Partitioning the list into {THREADS} files...")
    list_partitioner.start_splitting(FULL_LIST_PATH, THREADS, SEPARATED_LISTS_FOLDER)
    print ("The list has been partitioned, starting the threads...")
    ip_tester.start_threads(THREADS, f"./{SEPARATED_LISTS_FOLDER}/{FULL_LIST_PATH}")
