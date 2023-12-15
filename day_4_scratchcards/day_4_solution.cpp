#include <iostream>
#include <sstream>
#include <fstream>
#include <algorithm>
#include <numeric>
#include <utility>
#include <bitset>
#include <chrono>
#include<cmath>

struct ScratchCard { // number + count
    int cardNumber;
    std::vector<int> winningNumbers;
    std::vector<int> myNumbers;
    int matches;
    int copies = 1;

    void addNumbersToBitset(std::string numberStr, bool isWinning){
        std::istringstream iss(numberStr);
        int number;
        while (iss >> number) {
            if (isWinning) {
                winningNumbers.push_back(number);
            } else {
                myNumbers.push_back(number);
            }
        }
    }
    
    void addPoints(int &currentPoints) {
        std::sort(winningNumbers.begin(), winningNumbers.end());
        std::sort(myNumbers.begin(), myNumbers.end());
        std::vector<int> commonNumbers;
        std::set_intersection(
            winningNumbers.begin(), winningNumbers.end(),
            myNumbers.begin(), myNumbers.end(),
            std::back_inserter(commonNumbers)
        );

        // Output common elements
        for (int num : commonNumbers) {
            std::cout << num << " ";
        }


        int cardPoints = 1 * pow(2,commonNumbers.size()-1);


        currentPoints += cardPoints;
        matches = commonNumbers.size();
    }


};

void parseCards(std::ifstream &inputFile, std::vector<ScratchCard> &cards) {
    std::string s_line;
    int cardNumber = 1;

    while (getline(inputFile, s_line)) {
        ScratchCard card;
        card.cardNumber = cardNumber;
        size_t colonPos = s_line.find(':');
        size_t barPos = s_line.find('|');

        if (colonPos != std::string::npos && barPos != std::string::npos) {
            std::string winningNumbersStr = s_line.substr(colonPos + 1, barPos - colonPos - 1);
            std::string myNumbersStr = s_line.substr(barPos + 1);

            card.addNumbersToBitset(winningNumbersStr, true);
            card.addNumbersToBitset(myNumbersStr, false);

            cards.push_back(card);
        }
        cardNumber++;
    }
}



int main() {
    auto start_time = std::chrono::high_resolution_clock::now();
    std::ifstream inputFile ("cards");

    if ( !inputFile.is_open() ) {
        std::cerr << "Failed to open the input file." << std::endl;
        return 1;
    }


    std::vector<ScratchCard> cards;
    parseCards(inputFile, cards);
    int score = 0;

    for (auto card = begin(cards); card != end(cards); ++card) {
        card->addPoints(score);
    }

    std::cout << "Part 1 :: Score: " << score << std::endl;


    // Part 2
    std::unordered_map<int,int> cardCopies;

    for (auto& card : cards) {
        cardCopies[card.cardNumber] = 1;// Initialize with 1 copy for each card
        std::cout << "Card#: " << card.cardNumber << " has copies: " << cardCopies[card.cardNumber] << std::endl;
    }

    // Now make a single pass through
    for (auto card = begin(cards); card != end(cards); ++card) {

        int cardIndex = card->cardNumber - 1;
        int matches = card->matches;
        std::cout << "Card#: " << card->cardNumber << ", # Matches: " << matches << ", # Copies: " << cardCopies[card->cardNumber] << std::endl;

        if (matches > 0) {
            for (int i=1; i<=matches;i++) {
                    int cardWon = card->cardNumber+i;
                    std::cout << "Card #:" << card->cardNumber << " wins " << cardCopies[card->cardNumber] << " copies of card#: " << cardWon << std::endl;
                    cardCopies[cardWon] += cardCopies[card->cardNumber];
                    
            }
            continue;
        }

    }
    
    int totalCards = 0;
    for (int i=0; i<cardCopies.size(); i++) {
        totalCards += cardCopies[i];
    }
    
    std::cout << "Part 2 :: Total scratchcards: " << totalCards << std::endl;


    inputFile.close();
    // Stop the timer
    auto end_time = std::chrono::high_resolution_clock::now();

    // Calculate the elapsed time
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);

    // Print the elapsed time in milliseconds
    std::cout << "Execution Time: " << duration.count() << " milliseconds" << std::endl;

    return 0;
}