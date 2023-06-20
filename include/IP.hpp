#pragma once
#include <string>

class IP
{
public:
    IP(const std::string& ip);
    IP(const IP& ip);
    IP(int32_t ip);
    ~IP();

    int32_t getIP() const;
    void setIP(const std::string& ip);
    void setIP(const IP& ip);
    void setIP(int32_t ip);
    std::string toString() const;
    int32_t toInt(const std::string& ip) const;
    bool isPortUp(int port, timeval timeoutval) const;
    int ping(timeval timeoutval) const;
    std::string getHostname();

    int32_t operator++(int);
    bool operator==(const IP& ip) const;

private:
    int32_t ip;
    std::string hostname;
};