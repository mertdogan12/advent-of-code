#include "iostream"
#include "fstream"
#include "vector" 
#include "array"
#include "string"
#include "algorithm"

#define Log(x) std::cout << x << std::endl;
#define LogVec(x) for (std::string elem: x) std::cout << elem << ' '; std::cout << std::endl;
#define LogVecInt(x) for (int elem: x) std::cout << elem << ' '; std::cout << std::endl;

void LogBoard(const std::array<std::array<int, 5>, 5> board)
{
    for(std::array<int, 5> arr: board)
    {
        LogVecInt(arr);
    }
}

int calcScore(std::array<std::array<int, 5>, 5> board, int bingoNum)
{
    int sum = 0;
    for (std::array<int, 5> line: board)
        for (int number: line)
            if (number != -1)
                sum += number;

    return bingoNum * sum;
}

std::vector<std::string> splitString(std::string inp, char at)
{
    std::string s = "";
    std::vector<std::string> out;

    for (char c: inp)
    {
        if (c == at)
        {
            if (s == "")
                continue;

            out.push_back(s);
            s = "";
        } else
            s.push_back(c);
    }

    out.push_back(s);

    return out;
}

int main() 
{
    std::string input;
    std::ifstream inputFile("input.txt");

    std::vector<int> bingoNumbers;
    std::vector<std::array<std::array<int, 5>, 5>> boards;

    int line = 0;
    int index = 0;
    std::array<std::array<int, 5>, 5> current;
    while (getline(inputFile, input))
    {
        line++;
        if (line == 1)
        {
            for (std::string str: splitString(input, ','))
                bingoNumbers.push_back(std::stoi(str));

            continue;
        }

        if (input == "")
        {
            if (line == 2) 
                continue;

            boards.push_back(current);
            index = 0;
            continue;
        }

        std::vector<std::string> curLine = splitString(input, ' ');

        for (int i = 0; i < curLine.size(); i++)
            current[index][i] = std::stoi(curLine[i]);
        
        index++;    
    }
    boards.push_back(current);

    int lastScore;
    std::vector<std::array<std::array<int, 5>, 5>*> winningBoards;
    for (int bingoNum: bingoNumbers)
    {
        for (std::array<std::array<int, 5>, 5> &board: boards)
        {
            std::array<std::array<int, 5>, 5> copy = board;

            for (int j = 0; j < board.size(); j++)
            {
                bool fullLine = true;
                bool found = false;
                for (int i = 0; i < board[j].size(); i++)
                {
                    if (board[j][i] == bingoNum)
                    {
                        board[j][i] = -1;
                        found = true;

                        bool fullVertLine = true;
                        for (int k = j; k >= 0; k--)
                            if (board[k][i] != -1)
                                fullVertLine = false;

                        for (int k = 0; k < board.size(); k++)
                            if (board[k][i] != -1)
                                fullVertLine = false;

                        if (!fullVertLine)
                            continue;

                        if (std::find(winningBoards.begin(), winningBoards.end(), &board) == winningBoards.end())
                        {
                            lastScore = calcScore(board, bingoNum);
                            winningBoards.push_back(&board);
                        }
                    }
                    else if (board[j][i] != -1)
                        fullLine = false;
                }

                if (fullLine && found) 
                {
                    if (std::find(winningBoards.begin(), winningBoards.end(), &board) == winningBoards.end())
                    {
                        lastScore = calcScore(board, bingoNum);
                        winningBoards.push_back(&board);
                    }
                }
            }
        }
    }

    Log(lastScore);

    return 1;
}
