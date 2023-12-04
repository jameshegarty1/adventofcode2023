#include <iostream>
#include <sstream>
#include <fstream>
#include <unordered_map>
#include <string>
#include <chrono>
#include <cmath>

using namespace std;

static std::unordered_map<std::string, int> maxColors = {
            {"red", 12},
            {"green", 13},
            {"blue", 14},
};

struct GameInfo {
    int gameID;
    std::unordered_map<std::string, int> maxCounts = {
        {"red", 0},
        {"green", 0},
        {"blue", 0},
    };

    void generateFromInputString(const std::string& line) {
        size_t colonPos = line.find(":");

        if (colonPos != string::npos) {
            string gameIDString = line.substr(0, colonPos);
            istringstream IDInfo(gameIDString);
            int ID;
            std::string game;
            if (IDInfo >> game >> ID) {
                gameID = ID;
            }

            string gameInfo = line.substr(colonPos +1);
            istringstream revealStream(gameInfo);
            string reveal;

            while (std::getline(revealStream, reveal, ';')) {

                std::istringstream colorStream(reveal);
                std::string colorCount;

                while (std::getline(colorStream, colorCount, ',')) {
                    std::istringstream colorInfo(colorCount);
                    int count;
                    std::string color;
                
                    if (colorInfo >> count >> color) {

                        std::unordered_map<std::string, int>::iterator it = maxCounts.find(color);
                        if (it != maxCounts.end()) {
                            if (count > it->second) {
                                maxCounts[color] = count;
                            }
                        }
                    }
                }
            }
        }
    }
};


bool isGamePossible(GameInfo gameInfo) {
    bool possible = true;
    for (const auto& pair : gameInfo.maxCounts) {
        std::unordered_map<std::string, int>::iterator it = maxColors.find(pair.first);
    
        if (pair.second > it->second) {
            possible = false;
        }

    }
    return possible;
}

int calculateCubic(GameInfo gameInfo){
    int redValue = gameInfo.maxCounts["red"];
    int greenValue = gameInfo.maxCounts["green"];
    int blueValue = gameInfo.maxCounts["blue"];

    return redValue * greenValue * blueValue;
}

int main() {
    auto start_time = std::chrono::high_resolution_clock::now();
    ifstream inputFile ("elf_game_records");

    if ( !inputFile.is_open() ) {
        cerr << "Failed to open the input file." << std::endl;
        return 1;
    }

    std::string s_line;
    int sum_game_ids = 0;
    int sum_cubic = 0;

    while (getline(inputFile, s_line)) {
        GameInfo game;
        game.generateFromInputString(s_line);

        //Part 1
        if (isGamePossible(game)) {
            sum_game_ids += game.gameID;
        }

        //Part 2
        int cubic = calculateCubic(game);
        sum_cubic += cubic;


    }

    cout << "Part 1: Sum of IDs from possible games: " << sum_game_ids << endl;
    cout << "Part 2: Sum of cubics from  games: " << sum_cubic << endl;

    // Part 2

    inputFile.close();
    // Stop the timer
    auto end_time = std::chrono::high_resolution_clock::now();

    // Calculate the elapsed time
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);

    // Print the elapsed time in milliseconds
    std::cout << "Execution Time: " << duration.count() << " milliseconds" << std::endl;

    return 0;
}