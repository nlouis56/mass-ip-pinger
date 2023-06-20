#include <thread>
#include <fstream>
#include <sstream>
#include <iostream>

#include "IP.hpp"
#include "scanning.hpp"
#include "logger.hpp"

#define INTERESTING_PORTS {22, 80, 443, 445, 515, 554, 3389, 5357, 5900, 5985, 6000, 6379, 6969, 8080, 8443, 9100}

void ipVectorMaker(std::string listPath, std::vector<IP>& ips)
{
    std::ifstream listFile(listPath);
    std::string strIp;

    if (!listFile.is_open())
    {
        std::cout << "Error: Unable to open file" << std::endl;
        error("Unable to open file " + listPath + " in ipVectorMaker()");
        return;
    } else {
        debug("File " + listPath + " opened");
        while (std::getline(listFile, strIp))
        {
            if (strIp.empty())
                continue;
            try {
                ips.push_back(IP(strIp));
            } catch (std::exception& e) {
                error(e.what());
                continue;
            }
        }
        debug("Wrote " + std::to_string(ips.size()) + " IPs to vector");
        listFile.close();
        debug("File " + listPath + " closed");
    }
}

void ip_tester(std::string listPath, std::string outPath)
{
    std::vector<IP> ips;
    std::string strIp;
    std::stringstream ssBatchWrite;
    struct timeval timeout;
    timeout.tv_sec = 0;
    timeout.tv_usec = 500000;

    // get ips from file into vector
    ipVectorMaker(listPath, ips);
    // for each ip in vector
    for (auto &ip : ips)
    {
        int latency = ip.ping(timeout);
        //debug("Pinging " + ip.toString() + " with timeout " + std::to_string(timeout.tv_usec) + "ms. Latency: " + std::to_string(latency) + "ms");
        if (latency != -1)
        {
            //debug(ip.toString() + " is up, latency: " + std::to_string(latency) + "ms");
            std::vector<int> open_ports;
            struct timeval portTimeout;
            portTimeout.tv_sec = 0;
            portTimeout.tv_usec = (latency + 50) * 1000;
            for (int port : INTERESTING_PORTS)
            {
                if (ip.isPortUp(port, portTimeout))
                {
                    open_ports.push_back(port);
                }
            }
            ssBatchWrite << ip.toString() << ";" << ip.getHostname() << ";";
            for (int i = 0; i < (int)open_ports.size(); i++)
            {
                ssBatchWrite << open_ports[i];
                if (i != (int)open_ports.size() - 1)
                {
                    ssBatchWrite << ",";
                }
            }
            ssBatchWrite << std::endl;
            // if the string is too long, write to file
            if (ssBatchWrite.str().length() > 512)
            {
                batchIptable(ssBatchWrite.str(), outPath);
                debug(std::to_string(ssBatchWrite.str().length()) + " characters written to " + outPath);
                ssBatchWrite.str("");
            }
        }
    }
    // write remaining ips to file
    batchIptable(ssBatchWrite.str(), outPath);
    debug(std::to_string(ssBatchWrite.str().length()) + " remaining characters written to " + outPath);
    debug("ip_tester() finished on file " + listPath);
    std::cout << "Finished testing IPs in " << listPath << std::endl;
    std::ifstream inFile(outPath);
    int up_ips_count = std::count(std::istreambuf_iterator<char>(inFile), std::istreambuf_iterator<char>(), '\n');
    debug(std::to_string(up_ips_count) + " IPs are up from" + std::to_string(listPath.size()) + " IPs");
    return;
}
