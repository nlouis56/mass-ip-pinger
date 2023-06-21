#include <iostream>
#include <fstream>
#include <string>
#include <stdexcept>
#include <vector>
#include <thread>

#include "IP.hpp"
#include "scanning.hpp"
#include "logger.hpp"
#include "listTools.hpp"

void ip_tester(std::vector<std::string> ipStrings, std::string outPath);

static void loadMegaVector(std::vector<std::string> &inputPath, std::vector<std::string> &megaVector)
{
    for (auto &path : inputPath)
    {
        std::vector<std::string> tempVector = load_file(path);
        if (tempVector.size() + megaVector.size() > megaVector.max_size())
        {
            std::cout << "Shit's gonna explode if " + path + " is loaded, not loading anymore ips to scan";
            break;
        } else {
            megaVector.insert(megaVector.end(), tempVector.begin(), tempVector.end());
        }
    }
    std::sort(megaVector.begin(), megaVector.end());
    megaVector.erase(std::unique(megaVector.begin(), megaVector.end()), megaVector.end());
}

static void startThreads(std::vector<std::string> inputPath, std::string outputPathBase, int threadCount)
{
    std::vector<std::thread> threads;
    std::vector<std::string> biggestVectorOnEarthOMFG;
    loadMegaVector(inputPath, biggestVectorOnEarthOMFG);
    int threadIndex = 0;
    int ipsPerThread = biggestVectorOnEarthOMFG.size() / threadCount;
    int ipsLeft = biggestVectorOnEarthOMFG.size() % threadCount;
    debug("will load " + std::to_string(ipsPerThread) + " per thread plus " + std::to_string(ipsLeft) + " ips remaining");
    for (int i = 0; i < threadCount; i++)
    {
        std::vector<std::string> threadIps;
        for (int j = 0; j < ipsPerThread; j++)
        {
            threadIps.push_back(biggestVectorOnEarthOMFG[threadIndex]);
            threadIndex++;
        }
        if (ipsLeft > 0)
        {
            threadIps.push_back(biggestVectorOnEarthOMFG[threadIndex]);
            threadIndex++;
            ipsLeft--;
        }
        threads.push_back(std::thread(ip_tester, threadIps, outputPathBase + std::to_string(i)));
        debug("thread " + std::to_string(i) + " started with " + std::to_string(threadIps.size()) + " ips");
    }
    for (auto &thread : threads)
    {
        thread.join();
    }
}

bool arePathsValid(std::vector<std::string> listsPaths)
{
    for (int i = 0; i < (int)listsPaths.size(); i++)
    {
        std::ifstream file(listsPaths[i]);
        if (!file.good())
        {
            return false;
        }
    }
    return true;
}

std::vector<std::string> getFlags(int argc, char** argv)
{
    std::vector<std::string> flags;
    for (int i = 0; i < argc; i++)
    {
        if (argv[i][0] == '-')
        {
            flags.push_back(argv[i]);
        }
    }
    return flags;
}

std::vector<std::string> getArgs(int argc, char** argv)
{
    std::vector<std::string> args;
    for (int i = 1; i < argc; i++)
    {
        if (argv[i][0] != '-')
        {
            std::string arg = argv[i];
            args.push_back(arg);
        }
    }
    return args;
}

int main(int argc, char** argv)
{
    std::vector<std::string> flags = getFlags(argc, argv);
    std::vector<std::string> args = getArgs(argc, argv);
    bool useUnwrapped = false;
    bool useListMaker = false;
    std::vector<std::string> listsPaths;
    bool runTests = false;
    int threadCount = -1;

    for (int i = 0; i < (int)flags.size(); i++)
    {
        if (flags[i] == "--use-unwrapped" || flags[i] == "-uu")
        {
            useUnwrapped = true;
            try {
                listsPaths.emplace_back(args[i]);
                debug("Added " + args[i] + " to listsPaths");
            } catch (std::out_of_range& e) {
                std::cout << "Error: No file specified for --use-unwrapped" << std::endl;
                error("No file specified for --use-unwrapped --- CRITICAL");
                exit(1);
            }
        }
        if (flags[i] == "--unwrap" || flags[i] == "-u")
        {
            useListMaker = true;
            try {
                listsPaths.emplace_back(args[i]);
                debug("Added " + args[i] + " to listsPaths");
            } catch (std::out_of_range& e) {
                std::cout << "Error: No file specified for --unwrap" << std::endl;
                error("No file specified for --unwrap --- CRITICAL");
                exit(1);
            }
        }
        if (flags[i] == "--help" || flags[i] == "-h")
        {
            std::cout << "Usage: " << argv[0] << " [flags] [args]" << std::endl;
        }
        if (flags[i] == "--run-tests" || flags[i] == "-rt")
        {
            runTests = true;
            if (i < (int)args.size()) {
                try {
                    threadCount = std::stoi(args[i]);
                } catch (std::invalid_argument& e) {
                    std::cout << "Error: Invalid thread count" << std::endl;
                }
            }
        }
    }
    if (runTests == true && threadCount <= 0)
    {
        std::cout << "Thread count invalid or not specified, defaulting to 1" << std::endl;
        debug("Thread count invalid or not specified, defaulting to 1");
        threadCount = 1;
    }
    if (!arePathsValid(listsPaths))
    {
        std::cout << "Error: One or more paths are invalid" << std::endl;
        error("One or more paths are invalid --- CRITICAL");
        exit(1);
    }
    std::vector<std::string> fullListsPaths;
    if (useUnwrapped || useListMaker) {
        fullListsPaths = unwrapListsIfNecessary(listsPaths);
        if ((int)fullListsPaths.size() == 0)
        {
            std::cout << "Error: No valid lists were found, no lists were created" << std::endl;
            error("No valid lists were found, no lists were created --- CRITICAL");
            exit(1);
        }
        std::cout << "Successfully unwrapped " << fullListsPaths.size() << " lists" << std::endl;
    }
    if (runTests == true)
    {
        if ((int)fullListsPaths.size() == 0)
        {
            std::cout << "Error: No lists to test" << std::endl;
            error("No lists to test --- CRITICAL");
            exit(1);
        }
        startThreads(fullListsPaths, "testedIps", threadCount);
        exit(0);
    }
    return 0;
}
