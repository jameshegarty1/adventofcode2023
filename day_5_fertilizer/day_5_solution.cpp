#include <fstream>
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <tuple>
#include <sstream>

std::vector<std::string> splitFileIntoBlocks(const std::string& filePath) {
    std::ifstream file(filePath);
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string content = buffer.str();

    std::vector<std::string> blocks;
    size_t pos = 0;
    std::string token;
    std::string delimiter = "\n\n";

    // Use find and substr to split the string by "\n\n"
    while ((pos = content.find(delimiter)) != std::string::npos) {
        token = content.substr(0, pos);
        blocks.push_back(token);
        content.erase(0, pos + delimiter.length());
    }
    blocks.push_back(content); // Add the last part

    return blocks;
}

std::vector<long> extractSeeds(const std::string& str) { //For Part 1
    std::vector<long> numbers;
    size_t i = 0;

    while (i < str.length()) {
        // Check if the current character is a digit
        if (std::isdigit(str[i])) {
            std::size_t start = i;
            // Move forward in the string until a non-digit character is found
            while (i < str.length() && std::isdigit(str[i])) {
                ++i;
            }
            // Convert the substring that represents the number to an longeger
            long number = std::stol(str.substr(start, i - start));
            numbers.push_back(number);
        } else {
            ++i; // Move to the next character if not a digit
        }
    }

    return numbers;
}

std::vector<std::pair<long,long>> extractSeedsAsTuples(const std::string& str) { //For Part 2
    std::vector<std::pair<long,long>> vSeedPairs;
    size_t i = 0;

    std::pair<long,long> seedPair;

    std::vector<long> seedVec;

    while (i < str.length()) {
        if ( seedVec.size()  == 2 ) {
            seedPair = std::make_pair(seedVec[0], seedVec[0] + seedVec[1]);
            vSeedPairs.push_back(seedPair);
            seedVec.clear();
        }
        // Check if the current character is a digit
        if (std::isdigit(str[i])) {
            std::size_t start = i;
            // Move forward in the string until a non-digit character is found
            while (i < str.length() && std::isdigit(str[i])) {
                ++i;
            }
            // Convert the substring that represents the number to an longeger
            long number = std::stol(str.substr(start, i - start));
            seedVec.push_back(number);
        } else {
            ++i; // Move to the next character if not a digit
        }
    }
    //handle the last pair
    if ( seedVec.size()  == 2 ) {
            seedPair = std::make_pair(seedVec[0], seedVec[0] + seedVec[1]);
            vSeedPairs.push_back(seedPair);
            seedVec.clear();
    }

    return vSeedPairs;
}

std::vector<std::vector<std::tuple<long,long,long>>> buildMaps(std::vector<std::string> blocks) {
    std::vector<std::vector<std::tuple<long,long,long>>> vMaps;


    for (auto & block : blocks) {
        std::vector<std::tuple<long,long,long>> vMap;
        std::stringstream ss;
        ss << block;
        std::string line;

        std::getline(ss, line); //skip first line

        while ( std::getline(ss, line) ) {
            if (!line.empty()) {
                std::istringstream lineStream(line);
                std::tuple<long,long,long> tMap;
                long a, b, c;

                lineStream >> a >> b >> c;

                tMap = std::make_tuple(a, b, c);


                vMap.push_back(tMap);
            }
        }

        vMaps.push_back(vMap);
    }

    return vMaps;
}

void solvePartOne(const std::string& filePath){
    std::vector<std::string> blocks = splitFileIntoBlocks(filePath);

    auto sSeeds = blocks[0]; //grab seeds
    blocks.erase(blocks.begin()); //remove seeds from blocks

    auto vSeeds = extractSeeds(sSeeds);

    auto vMaps = buildMaps(blocks);

    // loop through maps, each time updating "vSeeds" with the matched value
    // at the end of this vSeeds is effectively transformed longo "vLocations"
    for (auto map: vMaps) {
        long seedInd = 0;
        for (auto seed: vSeeds) {
            long mappedValue = -1;
            for (auto subMap: map) {
                long destStart = std::get<0>(subMap);
                long sourceStart = std::get<1>(subMap);
                long length = std::get<2>(subMap);

                if ( seed >= sourceStart && seed <= sourceStart+length ) {
                    mappedValue = destStart + (seed - sourceStart);
                    break;
                }

            }

            if ( mappedValue == -1 ) {
                mappedValue = seed;
            }

            vSeeds[seedInd] = mappedValue; 

        seedInd++;    
        }
    }

    for (auto seed: vSeeds) {
        std::cout << "Seed mapped to location: " << seed << std::endl;
    }

    long minValue = vSeeds[0];

    for (auto seed: vSeeds) {
        if ( seed < minValue ) {
            minValue = seed;
        }
    }


    std::cout << "Part 1 - Smallest location is: " << minValue << std::endl;
    
}


void solvePartTwo(const std::string& filePath){
    std::vector<std::string> blocks = splitFileIntoBlocks(filePath);


    auto sSeeds = blocks[0]; //grab seeds
    blocks.erase(blocks.begin()); //remove seeds from blocks

    auto vSeeds = extractSeedsAsTuples(sSeeds);

    for (auto seedRange : vSeeds) {
        std::cout << "Seed range start = " << seedRange.first << " seed range end = " << seedRange.second << std::endl;
    }

    auto vMaps = buildMaps(blocks);


    for (auto map: vMaps) {

        std::vector<std::pair<long,long>> mappedRanges;

        while ( vSeeds.size() > 0 ) {
            auto seeds = vSeeds.back(); // take the last seed range
            vSeeds.pop_back(); // pop off the last seed range
            
            //we need to find how the range of seeds maps to soil and then on. There will probably be a few segments of matches

            long seedStart = seeds.first;
            long seedEnd = seeds.second;

            bool mapped = false;

            
            for ( auto subMap: map ) {

                long destStart = std::get<0>(subMap);
                long sourceStart = std::get<1>(subMap);
                long rangeSize = std::get<2>(subMap);

                long overlapStart = std::max(seedStart, sourceStart);
                long overlapEnd = std::min(seedEnd, sourceStart + rangeSize);
                
                //case when there is an overlap 
                if (overlapStart < overlapEnd) {
                    long mappedRangeStart = destStart + (overlapStart - sourceStart);
                    long mappedRangeEnd = destStart + (overlapEnd - sourceStart);

                    std::cout << "Mapped source range (" << seedStart << "," << seedEnd << ") to dest range (" << mappedRangeStart << "," << mappedRangeEnd << ")" << std::endl; 

                    mappedRanges.push_back(std::make_pair(mappedRangeStart, mappedRangeEnd));
                
                    //handle case where the overlap is only a subset of the seeds. We need to add the ranges either side of the overlap back to vSeeds to attempt to re-map
                    if ( overlapStart > seedStart ) {
                        vSeeds.push_back(std::make_pair(seedStart, overlapStart));
                    }
                    if ( overlapEnd < seedEnd ) {
                        vSeeds.push_back(std::make_pair(overlapEnd, seedEnd));
                    }

                    mapped = true;

                    break; //we break the for-loop here because we found a map. If it wasnt a full map, we pushed back to vSeeds anyway.
                }
            }

            if ( mapped == false ){
                mappedRanges.push_back(std::make_pair(seedStart, seedEnd));
            }
        }
        vSeeds = mappedRanges;
    }

    long smallestLocation = std::get<0>(vSeeds[0]);

    for (auto seed: vSeeds) {
       long rangeStart =  std::get<0>(seed);
        if (rangeStart < smallestLocation) {
            smallestLocation = rangeStart;
        }
    }

    std::cout << "Part two - smallest location = " << smallestLocation << std::endl;
}


int main() {

    std::string filePath = "main_input";

    //solvePartOne(filePath);


    solvePartTwo(filePath);




    return 0;
}

