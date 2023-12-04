#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <map>

static std::unordered_map<std::string, int> wordToNumber = {
            {"one", 1},
            {"two", 2},
            {"three", 3},
            {"four", 4},
            {"five", 5},
            {"six", 6},
            {"seven", 7},
            {"eight", 8},
            {"nine", 9}
};

void parseLiteralDigits(const std::string& line, std::map<int, int>& map) {
    int idx = 0;
    while (idx < line.size()) {
        if (isdigit(line[idx])) {
            map.insert(std::make_pair(idx, line[idx] - '0'));
        }
        idx++;
    }
}

void parseWordsAsDigits(const std::string& line, std::map<int, int>& map) {
    for ( auto pair: wordToNumber){
        int pos = line.find(pair.first);
        while (pos != std::string::npos) {
            map.insert(std::make_pair(pos, pair.second));
            pos = line.find(pair.first, pos + pair.first.size());
        }
    }
}

int main() {
    std::ifstream inputFile ("file_to_decode");

    if ( !inputFile.is_open() ) {
        std::cerr << "Failed to open the input file." << std::endl;
        return 1;
    }

    std::string s_line;
    int i_totalValue = 0;

    while (std::getline(inputFile, s_line)) {
        std::map<int, int> map;
        parseLiteralDigits(s_line, map);
        parseWordsAsDigits(s_line, map);

        i_totalValue += map.begin()-> second * 10 + map.rbegin()->second;
    }
    inputFile.close();

    std::cout << "Total Sum: " << i_totalValue << std::endl;
    return 0;
}