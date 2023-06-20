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

void ip_tester(std::string listPath, std::string outPath);

static void start_threads(std::vector<std::string> inPath, std::string outPath, int thread_count)
{
    std::vector<std::thread> threads;
    if (thread_count > (int)inPath.size())
    {
        thread_count = (int)inPath.size();
    } else if (thread_count < 1)
    {
        thread_count = 1;
    }
    for (int i = 0; i < thread_count; i++)
    {
        std::cout << "Starting thread " << i << std::endl;
        threads.push_back(std::thread(ip_tester, inPath[i], outPath + std::to_string(i) + ".out"));
    }
    for (int i = 0; i < thread_count; i++)
    {
        threads[i].join();
    }
}

static void argumentManager(int argc, char** argv)
{
    for (int i = 0; i < argc; i++)
    {
        if (std::string(argv[i]) == "--help" || std::string(argv[i]) == "-h")
        {
            std::cout << "Usage: " << argv[0] << " [options]" << std::endl;
            exit(0);
        }
        if (std::string(argv[i]) == "--list-maker")
        {
            if (i + 2 >= argc)
            {
                std::cout << "Error: Not enough arguments for --list-maker" << std::endl;
                exit(1);
            }
            std::vector<std::string> fullListsPaths = listMaker(argv[i + 2], argv[i + 3], std::stoi(argv[i + 1]));
            start_threads(fullListsPaths, argv[i + 3], std::stoi(argv[i + 1]));
            exit(0);
        }
        if (std::string(argv[i]) == "--use-unwrapped")
        {
            if (i + 2 >= argc)
            {
                std::cout << "Error: Not enough arguments for --use-unwrapped" << std::endl;
                exit(1);
            }
            std::vector<std::string> inPath;
            std::string outPath = "testedIps";
            int thread_count = 1;
            if (argv[i + 1] != NULL)
            {
                try {
                    thread_count = std::stoi(argv[i + 1]);
                } catch (std::invalid_argument& e) {
                    std::cout << "Error: Invalid thread count" << std::endl;
                    exit(1);
                }
            }
            if (thread_count < 1)
            {
                std::cout << "Error: Invalid thread count" << std::endl;
                exit(1);
            }
            for (int j = i + 2; j < argc; j++)
            {
                inPath.push_back(argv[j]);
            }
            start_threads(inPath, outPath, thread_count);
            exit(0);
        }
    }
}

int main(int argc, char** argv)
{
    argumentManager(argc, argv);
    std::cout << "Usage: " << argv[0] << " [options]" << std::endl;
    return 0;
}