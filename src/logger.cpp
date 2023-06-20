#include <string>
#include <iostream>
#include <fstream>
#include <sstream>
#include <ctime>
#include <vector>
#include "IP.hpp"

#define RED "\033[31m"
#define GREEN "\033[32m"
#define YELLOW "\033[33m"
#define RESET "\033[0m"

void error(const std::string& message)
{
    std::time_t t = std::time(nullptr);
    std::ofstream file;
    std::stringstream ss;

    ss << "[" << std::asctime(std::localtime(&t)) << "] - ERROR - " << message << std::endl;
    std::cout << RED << "[ERROR] - " << RESET << ss.str();
    file.open("error.log", std::ios::app);
    if (file.is_open())
    {
        file << ss.str();
        file.close();
    }
    else
    {
        std::cout << "Unable to open error.log" << std::endl;
    }
}

void debug(const std::string& message)
{
    std::ofstream file;
    std::time_t t = std::time(nullptr);
    std::string timestamp = std::asctime(std::localtime(&t));
    std::stringstream ss;

    timestamp.pop_back();
    ss << "[" << timestamp << "] - " << message << std::endl;
    file.open("debug.log", std::ios::app);
    if (file.is_open())
    {
        file << ss.str();
        file.close();
    }
    else
    {
        std::cout << "Unable to open debug.log" << std::endl;
        error("Unable to open debug.log in debug()");
    }
}

void iptable(IP& ip, std::vector<int>& open_ports, std::string const outPath="iptable.log")
{
    //std::time_t t = std::time(nullptr);
    std::stringstream ss;

    ss << ip.toString() << ";" << ip.getHostname() << ";";
    for (int i = 0; i < (int)open_ports.size(); i++)
    {
        ss << open_ports[i];
        if (i != (int)open_ports.size() - 1)
        {
            ss << ",";
        }
    }
    ss << std::endl;

    std::ofstream file(outPath, std::ios::app);
    if (file.is_open())
    {
        file << ss.str();
        file.close();
    }
    else
    {
        std::cout << "Unable to open " << outPath << std::endl;
        error("Unable to open " + outPath + " in iptable()");
    }
    file.close();
}

void batchIptable(std::string buffer, std::string outPath="iptable.log")
{
    std::ofstream file(outPath, std::ios::app);
    if (file.is_open())
    {
        file << buffer;
        file.close();
    }
    else
    {
        std::cout << "Unable to open " << outPath << std::endl;
        error("Unable to open " + outPath + " in batchIptable()");
    }
    file.close();
}