
#include <fstream>
#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <algorithm>
#include <cassert>

struct Round
{
    std::string opponent_choice;
    std::string my_choice;
};

std::vector<Round> parse_inputs(std::string file_path)
{
    std::vector<Round> res;
    std::ifstream f(file_path);
    std::string line;
    while(getline(f, line)) {
        std::stringstream liness(line);
        std::string opponent_choice_str, my_choice_str;
        liness >> opponent_choice_str;
        liness >> my_choice_str;
        res.push_back({opponent_choice_str, my_choice_str});
    }

    return res;
}

// X for Rock, Y for Paper, and Z for Scissors
// 1 for Rock, 2 for Paper, and 3 for Scissors)
// outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
// A for Rock, B for Paper, and C for Scissors
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win

const std::vector<std::pair<Round, int>> SCORE_MAP_1 = {
    {{"A","X"}, 3+1},
    {{"A","Y"}, 6+2},
    {{"A","Z"}, 0+3},
    {{"B","X"}, 0+1},
    {{"B","Y"}, 3+2},
    {{"B","Z"}, 6+3},
    {{"C","X"}, 6+1},
    {{"C","Y"}, 0+2},
    {{"C","Z"}, 3+3},
};

const std::vector<std::pair<Round, int>> SCORE_MAP_2 = {
    {{"A","X"}, 0+3},
    {{"A","Y"}, 3+1},
    {{"A","Z"}, 6+2},
    {{"B","X"}, 0+1},
    {{"B","Y"}, 3+2},
    {{"B","Z"}, 6+3},
    {{"C","X"}, 0+2},
    {{"C","Y"}, 3+3},
    {{"C","Z"}, 6+1},
};


int get_score(const std::vector<std::pair<Round, int>>& score_map, const Round& round)
{
    // Erf
    auto it = std::find_if(
        score_map.begin(),
        score_map.end(),
        [round](const std::pair<Round, int>& e){ return (e.first.opponent_choice == round.opponent_choice && e.first.my_choice == round.my_choice);});

    assert(it!=score_map.end()); // Meuh
    return it->second;
}

int main()
{
    auto rounds = parse_inputs("input");

    int score_1 = 0;
    for(const auto& round : rounds) {
        score_1 += get_score(SCORE_MAP_1, round);
    }
    std::cout << "score_1 = " << score_1 << std::endl;

    int score_2 = 0;
    for(const auto& round : rounds) {
        score_2 += get_score(SCORE_MAP_2, round);
    }
    std::cout << "score_2 = " << score_2 << std::endl;

    return 0;
}

