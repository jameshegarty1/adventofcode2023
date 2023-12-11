#include <iostream>
#include <fstream>
#include <chrono>
#include <vector>
#include <unordered_set>
#include <tuple>
#include <cctype>
#include <numeric>
#include <optional>
#include <algorithm>

static const std::vector<std::pair<int, int>> gridOffsets = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}, {-1, -1}, {-1, 1}, {1, -1}, {1, 1}}; // Top, Bottom, Left, Right


class EngineSchematic {
    private:
        std::string filename_;
        std::vector<std::vector<char>> data_;
        std::vector<std::pair<int,int>> digits_; //tuple is like row idx, column idx
        std::vector<std::pair<int,int>> specialSymbols_;
        std::vector<std::pair<int,int>> potentialGears_;
        struct Part {
            int number;
            std::vector<std::pair<int, int>> digitIndices;
        };

        std::vector<Part> parts_;

        struct Gear {
            std::pair<int, int> index;
            std::pair<Part, Part> adjacentParts;
            int gearRatio;
        };

        std::vector<Gear> gears_;

        std::optional<Part> isPartNumber(const std::pair<int, int>& element)  {
            for (const auto& part : parts_) {
                for (const auto& index : part.digitIndices) {
                    if (index == element) {
                        return part;
                    }
                }  
            }
            return std::nullopt;
        }

        bool checkAdjacentSymbol(int i, int j) const {

            for (const auto& offset : gridOffsets) {
                // check if offset is in bounds
                if (i + offset.first >= 0 && i + offset.first < data_.size() &&
                    j + offset.second >= 0 && j + offset.second < data_[i + offset.first].size()) {
                    if ( isSpecialSymbol(data_[i+offset.first][j+offset.second]) ) {
                        return true;
                    }
                }
            }
            return false;
        }

        bool findAdjacentParts(const std::pair<int, int>& element) {
            int i = element.first;
            int j = element.second;
            std::vector<Part> adjacentParts;

            for (const auto& offset : gridOffsets) {
                if (i + offset.first >= 0 && i + offset.first < data_.size() &&
                    j + offset.second >= 0 && j + offset.second < data_[i + offset.first].size()) {
                    const std::pair<int, int> offsetElement = std::make_pair(i + offset.first, j + offset.second);
                    auto partOpt = isPartNumber(offsetElement);
                    // if it is a part number, but is not already accounteed for
                    if (partOpt.has_value()) {
                        int partNumber = partOpt.value().number;
                        if (std::find_if(adjacentParts.begin(), adjacentParts.end(), 
                        [partNumber](const Part& part) { return part.number == partNumber; }) == adjacentParts.end()) {
                            adjacentParts.push_back(partOpt.value());
                }
                    }
                }
            }

            if (adjacentParts.size() == 2) {
                Gear gear;
                gear.index = element;
                gear.adjacentParts = std::make_pair(adjacentParts[0], adjacentParts[1]);
                gear.gearRatio = adjacentParts[0].number * adjacentParts[1].number;
                gears_.push_back(gear);
                return true;
            }
            return false;
        }
        
        bool isDigit(char c) const {
            return c >='0' && c <= '9';
        }

        bool isSpecialSymbol(char c) const {
            return !isDigit(c) && c != '.';
        }

        bool isGearType(char c) const {
            return c == '*';
        }

    public:
        EngineSchematic(const std::string& filename) : filename_(filename) {}

        bool ParseFile() {
            try {
                std::ifstream inputFile(filename_);
                if (!inputFile.is_open()) {
                    std::cerr << "Failed to open the input file." << std::endl;
                    return false;
                }

                std::string line;
                int row_index = 0;
                while (std::getline(inputFile, line)) {
                    std::vector<char> gridRow;

                    for (int col_index = 0; col_index < line.size(); ++col_index) {
                        char element = line[col_index];
                        gridRow.push_back(element);

                        if (isDigit(element)) {
                            digits_.push_back({row_index, col_index});
                        }
                        else if (isSpecialSymbol(element)) {
                            specialSymbols_.push_back({row_index, col_index});
                            if (isGearType(element)) {
                                potentialGears_.push_back({row_index, col_index});
                            }
                        }
                    }
                    data_.push_back(gridRow);
                    row_index++;
                    
                }
                inputFile.close();
                return true;
            
            } catch (const std::exception& e) {
                std::cerr << "An error occurred: " << e.what() << std::endl;
                return false;
            }
        }

        void GeneratePartsList() {
            int digitHopper = 0;
            for (const auto& digit : digits_) { // i,j
                if (digitHopper > 0) {
                    digitHopper--;
                    continue;
                }

                if (checkAdjacentSymbol(digit.first, digit.second)) {
                    std::cout << "At location: (" << digit.first << "," << digit.second << ") " << data_[digit.first][digit.second] << " is adjacent to a special symbol!" << std::endl;

                    std::string partNumberStr;
                    partNumberStr += data_[digit.first][digit.second];

                    Part part;
                    part.digitIndices.push_back(digit);

                    int leftIndex = digit.second - 1;
                    int rightIndex = digit.second + 1;

                    while ( rightIndex < data_[digit.first].size() && isDigit(data_[digit.first][rightIndex]) ) {
                        partNumberStr += data_[digit.first][rightIndex];
                        part.digitIndices.push_back({digit.first, rightIndex});
                        rightIndex++;
                        digitHopper++;
                    }
                    while ( leftIndex >= 0 && isDigit(data_[digit.first][leftIndex]) ) {
                        partNumberStr = data_[digit.first][leftIndex] + partNumberStr; 
                        part.digitIndices.push_back({digit.first, leftIndex});
                        leftIndex--;
                    }

                    part.number = std::stoi(partNumberStr);
                    parts_.push_back(part);
                }

            }

        }

        void GenerateGearsList() {
            for (const auto& potentialGear : potentialGears_) {
                if (findAdjacentParts(potentialGear)) {
                    auto gear = gears_.back();

                    std::cout << "Gear at (" <<  gear.index.first << ", " << gear.index.second << 
                    ") with gear adjacent part numbers: " << gear.adjacentParts.first.number << 
                    ", " << gear.adjacentParts.second.number << ". Gear ratio = " << gear.gearRatio << std::endl;
                }
                }
            }

        void PrintData() const {
            for (const auto& row : data_) {
                for (char symbol : row) {
                    std::cout << symbol;
                }
                std::cout << std::endl;
            }
        }

        void PrintTypes() const {
            for (auto rowIt = data_.begin(); rowIt != data_.end(); ++rowIt) {
                int i = std::distance(data_.begin(), rowIt);
                for (auto colIt = rowIt->begin(); colIt != rowIt->end(); ++colIt) {
                    int j = std::distance(rowIt->begin(), colIt);
                    char ch = *colIt;

                    std::cout << "Character '" << ch << "' at (" << i << ", " << j << ") is a ";
                    if (isDigit(ch)) {
                        std::cout << "digit";
                    } else if (isSpecialSymbol(ch)) {
                        std::cout << "special symbol";
                        if (isGearType(ch)) {
                            std::cout << " and is also a gear!";
                        }
                    } else {
                        std::cout << "dot";
                    }
                    std::cout << std::endl;
                }
            }
        }

        void PrintParts() const {
            for (const auto& part : parts_) {
                std::cout << "Part Number: " << part.number << " - Indices: ";
                for (const auto& index : part.digitIndices) {
                    std::cout << "(" << index.first << ", " << index.second << ") ";
                }
                std::cout << std::endl;
            }
        }

        int SumOfPartNumbers() const {
            return std::accumulate(parts_.begin(), parts_.end(), 0,
                           [](int sum, const Part& part) { return sum + part.number; });
        }

        int SumOfGearRatios() const {
            return std::accumulate(gears_.begin(), gears_.end(), 0,
                           [](int sum, const Gear& gear) { return sum + gear.gearRatio; });
        }
};


int main() {
    auto start_time = std::chrono::high_resolution_clock::now();

    EngineSchematic schematic("engine_schematic");


    
    if (schematic.ParseFile()) {
        //Part 1
        schematic.PrintData();
        schematic.PrintTypes();
        schematic.GeneratePartsList();
        schematic.PrintParts();
        std::cout << "Sum of part numbers = " << schematic.SumOfPartNumbers() << std::endl;

        //Part 2
        schematic.GenerateGearsList();
        std::cout << "Sum of gear ratios = " << schematic.SumOfGearRatios() << std::endl;

    }

    // Stop the timer
    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);
    std::cout << "Execution Time: " << duration.count() << " milliseconds" << std::endl;

    return 0;
}