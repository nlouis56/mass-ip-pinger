import scripts.ip_maker as ip_maker
import scripts.list_partitioner as list_partitioner
import scripts.ip_tester as ip_tester
import os
import sys

THREADS = 16
EXECUTING_PATH = os.path.dirname(os.path.realpath(__file__))
IP_TABLE_NAME = "french_ip_partial"
FULL_LIST_NAME = "french_ip_partial_unwrapped"
SEPARATED_LISTS_FOLDER = "cut_list"

def start_whole_program():
    print (f"Starting with {THREADS} threads\n")
    lines_written = ip_maker.make_full_list(os.path.join(EXECUTING_PATH, IP_TABLE_NAME), output_path=FULL_LIST_NAME)
    print (f"\nFull list has been created, {lines_written} lines written")
    print (f"Partitioning the list into {THREADS} files...\n")
    list_partitioner.start_splitting(FULL_LIST_NAME, THREADS, SEPARATED_LISTS_FOLDER)
    print ("\nThe list has been partitioned, starting the threads...\n")
    ip_tester.start_threads(THREADS, os.path.join(EXECUTING_PATH, SEPARATED_LISTS_FOLDER))


def get_env_variables():
    table_name = input(f"Enter the name of the table to use (default: {IP_TABLE_NAME}): ")
    if table_name != "":
        IP_TABLE_NAME = table_name
    full_list_name = input(f"Enter the name of the full list to create (default: {FULL_LIST_NAME}): ")
    if full_list_name != "":
        FULL_LIST_NAME = full_list_name
    separated_lists_folder = input(f"Enter the name of the folder to create for the separated lists (default: {SEPARATED_LISTS_FOLDER}): ")
    if separated_lists_folder != "":
        SEPARATED_LISTS_FOLDER = separated_lists_folder
    threads = input(f"Enter the number of threads to use (default: {THREADS}): ")
    if threads != "":
        try :
            THREADS = int(threads)
        except ValueError:
            print ("Invalid number of threads, using default value")


def main():
    get_env_variables()
    if len(sys.argv) == 1:
        start_whole_program()
    elif len(sys.argv) == 2:
        match sys.argv[1]:
            case "-sl":
                print ("Skipping the list creation, starting with the partitioning...\n")
                print (f"Partitioning the list into {THREADS} files...\n")
                list_partitioner.start_splitting(FULL_LIST_NAME, THREADS, SEPARATED_LISTS_FOLDER)
                print ("\nThe list has been partitioned, starting the threads...\n")
                ip_tester.start_threads(THREADS, os.path.join(EXECUTING_PATH, SEPARATED_LISTS_FOLDER))
            case "-sp":
                print ("Skipping the list creation and the partitioning, starting with the threads...\n")
                ip_tester.start_threads(THREADS, os.path.join(EXECUTING_PATH, SEPARATED_LISTS_FOLDER))
            case "-nt":
                print ("Creating the whole list and partitionning it, skipping the threads...\n")
                lines_written = ip_maker.make_full_list(os.path.join(EXECUTING_PATH, IP_TABLE_NAME), output_path=FULL_LIST_NAME)
                print (f"\nFull list has been created, {lines_written} lines written")
                print (f"Partitioning the list into {THREADS} files...\n")
                list_partitioner.start_splitting(FULL_LIST_NAME, THREADS, SEPARATED_LISTS_FOLDER)
            case "-h":
                print ("Usage: python run.py [option]\n")
                print ("Options:")
                print ("-sl\t\tSkip the list creation, start with the partitioning")
                print ("-sp\t\tSkip the list creation and the partitioning, start with the threads")
                print ("-nt\t\tCreate the whole list and partition it, skip the threads")
                print ("-h\t\tShow this help message")


if __name__ == "__main__":
    main()
