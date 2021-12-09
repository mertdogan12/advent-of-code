#include "fstream"
#include "string"
#include "iostream"

int main()
{
    std::string input;
    int depth = 0;
    int hp = 0;

    std::ifstream InputFile("input-day2.txt");

    while(getline(InputFile, input))
    {
        std::string action = input.substr(0, input.find(' '));
        int value = std::stoi(input.substr(input.find(' '), input.length() - 1));

        if (action == "forward")
            hp += value;
        else if (action == "down")
            depth += value;
        else if (action == "up")
            depth -= value;

        std::cout << action << " " << value << std::endl;
    }

    std::cout << "\n" << hp * depth << std::endl;
    
    return 1;
}
