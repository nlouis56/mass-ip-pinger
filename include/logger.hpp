#include <iostream>
#include <fstream>
#include <string>
#include <vector>

#include "IP.hpp"

void debug(const std::string& message);
void error(const std::string& message);
void iptable(IP& ip, std::vector<int>& open_ports, std::string const outPath="iptable.log");
void batchIptable(std::string buffer, std::string outPath="iptable.log");
