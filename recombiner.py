import sys
import os

outpath = 'recombined.out'

if __name__ == '__main__':
    dirfiles = os.listdir()
    dirfiles.sort()
    for file in dirfiles:
        if file.startswith('testedIps'):
            with open(file, 'r') as f:
                with open(outpath, 'a') as out:
                    for line in f:
                        out.write(line)