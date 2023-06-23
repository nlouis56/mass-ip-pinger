#include <string>
#include <vector>
#include <iostream>
#include <fstream>
#include <sstream>

#include "IP.hpp"
#include "logger.hpp"

static std::vector<std::string> split(std::string s, std::string delimiter) {
    size_t pos_start = 0, pos_end, delim_len = delimiter.length();
    std::string token;
    std::vector<std::string> res;

    while ((pos_end = s.find(delimiter, pos_start)) != std::string::npos) {
        token = s.substr (pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        res.push_back (token);
    }

    res.push_back (s.substr (pos_start));
    return res;
}

static std::vector<std::string> unwrap_line(std::string line)
{
    std::vector<std::string> unwrapped;
    std::vector<std::string> wrapped = split(line, ";");

    if (wrapped.size() != 3)
    {
        throw std::runtime_error("Invalid line");
    } else {
        IP start(wrapped[0]);
        IP end(wrapped[1]);
        while (start != end)
        {
            unwrapped.push_back(start.toString());
            start++;
        }
        unwrapped.push_back(end.toString());
        if ((int)unwrapped.size() != std::stoi(wrapped[2]))
        {
            // uncomment this if ip unwrapping does some weird shit
            //debug("Unwrapped size: " + std::to_string(unwrapped.size()) + " Expected size: " + wrapped[2]);
        }
    }
    return unwrapped;
}

std::vector<std::string> load_file(std::string filepath)
{
    std::ifstream file(filepath);
    std::vector<std::string> lines;
    std::string line;
    if (!file.is_open())
    {
        error("Unable to open file (" + filepath + ")");
        throw std::runtime_error("Unable to open file");
    }
    while (std::getline(file, line))
    {
        if (line.empty())
        {
            continue;
        }
        lines.push_back(line);
    }
    return lines;
}

static std::string unwrapSingularList(std::string path)
{
    std::vector<std::string> lines = load_file(path);
    std::ofstream out(path + ".full", std::ios::app);
    if (!out.is_open())
    {
        error("Unable to open output file (" + path + ")");
        throw std::runtime_error("Unable to open output file");
    }
    for (std::string line : lines)
    {
        std::vector<std::string> unwrapped = unwrap_line(line);
        debug("Writing " + std::to_string(unwrapped.size()) + " IPs to " + path + ".full");
        for (std::string ip : unwrapped)
        {
            out << ip << std::endl;
        }
    }
    return path + ".full";
}

static std::vector<std::string> unwrapSeparatedLists(std::vector<std::string> listPaths)
{
    std::vector<std::string> unwrappedListsPaths;
    for (std::string path : listPaths)
    {
        std::vector<std::string> lines = load_file(path);
        std::vector<std::string> unwrapped;
        for (std::string line : lines)
        {
            std::vector<std::string> unwrappedLine = unwrap_line(line);
            unwrapped.insert(unwrapped.end(), unwrappedLine.begin(), unwrappedLine.end());
        }
        std::ofstream out(path + ".full");
        if (!out.is_open())
        {
            error("Unable to open output file (" + path + ")");
            throw std::runtime_error("Unable to open output file");
        }
        for (std::string line : unwrapped)
        {
            out << line << std::endl;
        }
        out.close();
        unwrappedListsPaths.push_back(path + ".full");
    }
    return unwrappedListsPaths;
}

std::vector<std::string> unwrapListsIfNecessary(std::vector<std::string> listsPaths)
{
    std::vector<std::string> unwrappedListsPaths;
    for (std::string path : listsPaths)
    {
        std::vector<std::string> lines = load_file(path);
        bool toUnwrap = false;
        bool isValidUnwrapped = false;
        for (auto line : lines) {
            if (split(line, ";").size() == 3 || line.empty())
            {
                toUnwrap = true;
            } else {
                toUnwrap = false;
                break;
            }
        }
        if (toUnwrap)
        {
            std::string unwrapped = unwrapSingularList(path);
            unwrappedListsPaths.push_back(unwrapped);
            std::cout << "Unwrapped " << path << " to " << unwrapped << std::endl;
            continue;
        } else {
            for (auto line : lines)
            {
                try
                {
                    IP ip(line);
                    isValidUnwrapped = true;
                }
                catch (std::invalid_argument e)
                {
                    error("Invalid IP in list (" + path + ")");
                    std::cout << path << "is not a valid list, ignoring..." << std::endl;
                    isValidUnwrapped = false;
                    break;
                }
            }
            if (isValidUnwrapped)
            {
                std::cout << path << " is a valid list, using it as is" << std::endl;
                unwrappedListsPaths.push_back(path);
            }
        }
    }
    return unwrappedListsPaths;
}

std::vector<std::string> listMaker(std::string tablePath, std::string outPathBase, size_t filecount)
{
    std::vector<std::string> inputTable = load_file(tablePath);
    std::vector<std::string> outpaths;
    std::string fullTablePath = unwrapSingularList(tablePath);
    if (fullTablePath.empty())
    {
        error("Unable to unwrap table");
        throw std::runtime_error("Unable to unwrap table");
    }
    std::vector<std::string> fullTable = load_file(fullTablePath);
    std::sort(fullTable.begin(), fullTable.end());
    fullTable.erase(std::unique(fullTable.begin(), fullTable.end()), fullTable.end());
    size_t linesPerFile = fullTable.size() / filecount;
    size_t linesLeft = fullTable.size() % filecount;
    //debug("Lines in table: " + std::to_string(fullTable.size()));
    //debug("Lines per file: " + std::to_string(linesPerFile));
    //debug("Lines left: " + std::to_string(linesLeft));
    size_t currentLine = 0;
    for (size_t i = 0; i < filecount; i++)
    {
        std::ofstream out(outPathBase + std::to_string(i));
        if (!out.is_open())
        {
            error("Unable to open output file (" + outPathBase + std::to_string(i) + ")");
            throw std::runtime_error("Unable to open output file");
        }
        for (size_t j = 0; j < linesPerFile; j++)
        {
            out << fullTable[currentLine] << std::endl;
            currentLine++;
        }
        if (linesLeft > 0)
        {
            out << fullTable[currentLine] << std::endl;
            currentLine++;
            linesLeft--;
        }
        out.close();
        debug("Created file: " + outPathBase + std::to_string(i));
        debug("Lines in file: " + std::to_string(currentLine));
        outpaths.push_back(outPathBase + std::to_string(i));
    }
    return outpaths;
}
