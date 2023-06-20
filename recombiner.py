import sys
import os

outpath = 'recombined.out'

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: python3 recombiner.py <base_filepath>')
        sys.exit(1)
    else :
        base_filepath = sys.argv[1]
        dirfiles = os.listdir()
        dirfiles.sort()
        for file in dirfiles:
            if file.endswith('.out') and file.startswith(base_filepath):
                with open(file, 'r') as f:
                    with open(outpath, 'a') as out:
                        for line in f:
                            out.write(line)