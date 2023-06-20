#include <string>
#include <vector>
#include <iostream>

#include "IP.hpp"

std::vector<int> scanPorts(const IP& ip, int start=1, int end=65535)
{
    std::vector<int> open_ports;
    struct timeval timeoutval;
    timeoutval.tv_sec = 0;
    timeoutval.tv_usec = 500000;
    for (int i = start; i <= end; i++)
    {
        std::cout << "Scanning port " << i << std::endl;
        if (ip.isPortUp(i, timeoutval))
        {
            std::cout << "Port " << i << " is open" << std::endl;
            open_ports.push_back(i);
        }
    }
    return open_ports;
}
