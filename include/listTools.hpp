#pragma once

#include <string>
#include <vector>

std::vector<std::string> listMaker(std::string tablePath, std::string outPathBase, size_t filecount);
std::vector<std::string> unwrapListsIfNecessary(std::vector<std::string> listsPaths);
std::vector<std::string> load_file(std::string filepath);
