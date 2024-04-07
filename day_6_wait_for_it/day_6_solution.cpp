#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

struct Race {
    long time;
    long bestDistance;

    std::vector<long> p_Times;
    std::vector<long> p_Speeds;
    std::vector<long> p_Distances;

    long n_Wins;
};

std::vector<Race> parseRaces(const std::string& file) {
    std::ifstream data(file);
    std::vector<Race> races;
    std::string line;

    while (std::getline(data, line)) {
        std::istringstream timeStream(line);
        std::vector<std::string> columns;
        std::string column;

        std::string timeLabel;
        timeStream >> timeLabel; // Skip the "Time:" part in the data

        std::vector<long> times;

        long time;
        while (timeStream >> time) {
            times.push_back(time);
        }

        std::getline(data, line);

        std::istringstream distanceStream(line);
        std::string distanceLabel;
        distanceStream >> distanceLabel; // Skip the "Distance:" part in the data

        long distance;
        long index = 0;
        while (distanceStream >> distance) {
            Race race;
            race.time = times[index++];
            race.bestDistance = distance;
            races.push_back(race);
        }

    }
    return races;
}

void calculateRacePermutations(std::vector<Race>& races) {
    std::vector<long> raceWins;
    for (auto race: races) {
        std::cout << "Permuting race for time: " << race.time << " with bestDistance: " << race.bestDistance << std::endl;

        for (long i=0;i<race.time;i++) {
            long holdTime = i;
            long speed = i;
            long raceDuration = race.time - holdTime;
            long distanceCovered = speed * raceDuration;

            //std::cout << "DEBUG: HoldTime = " << holdTime << " Speed = " << speed << " Distance = " << distanceCovered << std::endl; 

            race.p_Times.push_back(holdTime);
            race.p_Speeds.push_back(speed);
            race.p_Distances.push_back(distanceCovered);

            if ( distanceCovered > race.bestDistance ) {
                //std::cout << "Race won with holdTime = " << holdTime << " with distance = " << distanceCovered << std::endl;
                race.n_Wins++;
            }
        }
        std::cout << "N race wins = " << race.n_Wins << std::endl;
        raceWins.push_back(race.n_Wins); 
    }

    long result = 1;
    for (auto element: raceWins) {
        result = result * element;
    }

    std::cout << "Part 1 - result = " << result << std::endl;
}

int main() {
   
    auto races = parseRaces("part_2_input");

    for (auto race: races ) {
        std::cout << "Race time = " << race.time << "ms, distance to beat = " << race.bestDistance << "mm" << std::endl; 
    }


    calculateRacePermutations(races);





}
