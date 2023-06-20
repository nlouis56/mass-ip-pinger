#include "IP.hpp"
#include <iostream>
#include <cstring>
#include <cstdlib>
#include <stdexcept>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <netinet/ip_icmp.h>
#include <sys/socket.h>
#include <sys/select.h>
#include <sys/time.h>
#include <fcntl.h>
#include <netdb.h>
#include <errno.h>
#include <unistd.h>
#include <chrono>

IP::IP(const std::string& ip)
{
    this->setIP(ip);
}

IP::IP(const IP& ip)
{
    this->setIP(ip);
}

IP::IP(int32_t ip)
{
    this->setIP(ip);
}

IP::~IP()
{
}

int32_t IP::getIP() const
{
    return this->ip;
}

void IP::setIP(const std::string& ip)
{
    this->ip = this->toInt(ip);
}

void IP::setIP(const IP& ip)
{
    this->ip = ip.getIP();
}

void IP::setIP(int32_t ip)
{
    this->ip = ip;
}

std::string IP::toString() const
{
    std::string ipString;
    ipString += std::to_string((this->ip >> 24) & 0xFF);
    ipString += ".";
    ipString += std::to_string((this->ip >> 16) & 0xFF);
    ipString += ".";
    ipString += std::to_string((this->ip >> 8) & 0xFF);
    ipString += ".";
    ipString += std::to_string(this->ip & 0xFF);
    return ipString;
}

int32_t IP::toInt(const std::string& ip) const
{
    int32_t ipInt = 0;
    int32_t octet = 0;
    int32_t octetCount = 0;
    for (int32_t i = 0; i < (int32_t)ip.length(); i++)
    {
        if (ip[i] == '.')
        {
            ipInt = (ipInt << 8) | octet;
            octet = 0;
            octetCount++;
        }
        else
            octet = (octet * 10) + (ip[i] - '0');
    }
    ipInt = (ipInt << 8) | octet;
    octetCount++;
    if (octetCount != 4)
        throw std::invalid_argument("Invalid IP address");
    return ipInt;
}

int32_t IP::operator++(int)
{
    return ++this->ip;
}

bool IP::operator==(const IP& ip) const
{
    return this->ip == ip.getIP();
}

bool IP::isPortUp(int port, timeval timeoutval) const
{
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        return false;
    }
    // Set the socket to non-blocking mode
    int flags = fcntl(sock, F_GETFL, 0);
    if (fcntl(sock, F_SETFL, flags | O_NONBLOCK) < 0) {
        close(sock);
        return false;
    }
    struct sockaddr_in serverAddress;
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_port = htons(port);
    if (inet_pton(AF_INET, this->toString().c_str(), &(serverAddress.sin_addr)) <= 0) {
        close(sock);
        return false;
    }
    int result = connect(sock, (struct sockaddr*)&serverAddress, sizeof(serverAddress));
    if (result < 0 && errno != EINPROGRESS) {
        close(sock);
        return false;
    }
    fd_set fdSet;
    FD_ZERO(&fdSet);
    FD_SET(sock, &fdSet);
    /* struct timeval timeout;
    timeout.tv_sec = 1; // Timeout in seconds
    timeout.tv_usec = 0; */
    result = select(sock + 1, nullptr, &fdSet, nullptr, &timeoutval);
    if (result < 0) {
        close(sock);
        return false;
    }
    else if (result == 0) {
        // Connection timeout
        close(sock);
        return false;
    }
    int error = 0;
    socklen_t errorLen = sizeof(error);
    if (getsockopt(sock, SOL_SOCKET, SO_ERROR, &error, &errorLen) < 0) {
        close(sock);
        return false;
    }
    if (error != 0) {
        close(sock);
        return false;
    }
    close(sock);
    return true;
}

std::string IP::getHostname()
{
    if (this->ip == 0)
        return "";
    if (this->hostname != "")
        return this->hostname;
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(this->ip);
    char hostname[NI_MAXHOST];
    if (getnameinfo((struct sockaddr*)&addr, sizeof(addr), hostname, sizeof(hostname), nullptr, 0, 0) != 0)
        return "";
    this->hostname = std::string(hostname);
    return std::string(hostname);
}

static uint16_t calculateChecksum(const void* addr, size_t len)
{
    uint32_t sum = 0;
    const uint16_t* ptr = static_cast<const uint16_t*>(addr);
    while (len > 1) {
        sum += *ptr++;
        len -= 2;
    }
    if (len == 1) {
        uint16_t data = 0;
        *(reinterpret_cast<uint8_t*>(&data)) = *(reinterpret_cast<const uint8_t*>(ptr));
        sum += data;
    }
    sum = (sum >> 16) + (sum & 0xffff);
    sum += (sum >> 16);
    return static_cast<uint16_t>(~sum);
}

int IP::ping(timeval timeoutVal) const {
    std::chrono::steady_clock::time_point startTime = std::chrono::steady_clock::now();

    int sock = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
    if (sock < 0) {
        //std::cout << "Socket error" << std::endl;
        return -1;
    }

    struct sockaddr_in serverAddress;
    std::memset(&serverAddress, 0, sizeof(serverAddress));
    serverAddress.sin_family = AF_INET;
    if (inet_pton(AF_INET, this->toString().c_str(), &(serverAddress.sin_addr)) <= 0) {
        //std::cout << "Invalid IP address" << std::endl;
        close(sock);
        return -1;
    }

    const int bufferSize = 128;  // Adjust the buffer size as needed
    char buffer[bufferSize];
    std::memset(buffer, 0, bufferSize);

    struct icmphdr icmpPacket;
    std::memset(&icmpPacket, 0, sizeof(icmpPacket));
    icmpPacket.type = ICMP_ECHO;
    icmpPacket.code = 0;
    icmpPacket.un.echo.id = getpid() & 0xFFFF;
    icmpPacket.un.echo.sequence = 0;
    icmpPacket.checksum = 0;
    icmpPacket.checksum = calculateChecksum(&icmpPacket, sizeof(icmpPacket));

    if (setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, reinterpret_cast<const char*>(&timeoutVal), sizeof(timeoutVal)) < 0) {
        //std::cout << "setsockopt < 0, IP down" << std::endl;
        close(sock);
        return -1;
    }

    if (sendto(sock, static_cast<const void*>(&icmpPacket), sizeof(struct icmphdr), 0,
               reinterpret_cast<struct sockaddr*>(&serverAddress), sizeof(serverAddress)) <= 0) {
        //std::cout << "sendto <= 0, IP down" << std::endl;
        close(sock);
        return -1;
    }

    struct sockaddr_in senderAddress;
    socklen_t senderAddressLen = sizeof(senderAddress);
    fd_set readSet;
    FD_ZERO(&readSet);
    FD_SET(sock, &readSet);

    int selectResult = select(sock + 1, &readSet, nullptr, nullptr, &timeoutVal);
    if (selectResult < 0) {
        //std::cout << "selectResult < 0, IP down" << std::endl;
        close(sock);
        return -1;
    } else if (selectResult == 0) {
        //std::cout << "selectResult == 0, IP down" << std::endl;
        close(sock);
        return -1;
    }

    if (recvfrom(sock, buffer, bufferSize, 0, reinterpret_cast<struct sockaddr*>(&senderAddress), &senderAddressLen) <= 0) {
        //std::cout << "recvfrom <= 0, IP down" << std::endl;
        close(sock);
        return -1;
    }

    struct iphdr* ipHeader = reinterpret_cast<struct iphdr*>(buffer);
    struct icmphdr* receivedPacket = reinterpret_cast<struct icmphdr*>(buffer + (ipHeader->ihl * 4));

    if (receivedPacket->type == ICMP_ECHOREPLY) {
        std::chrono::steady_clock::time_point endTime = std::chrono::steady_clock::now();
        close(sock);
        // Get duration in milliseconds
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime).count();
        //std::cout << "receivedPacket->type == ICMP_ECHOREPLY, IP up" << std::endl;
        return duration;
    }
    //std::cout << "receivedPacket->type != ICMP_ECHOREPLY, IP down" << std::endl;
    close(sock);
    return -1;
}