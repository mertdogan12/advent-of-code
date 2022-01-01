#include "iostream"
#include "fstream"
#include "vector"
#include "string"

unsigned long* splitString(std::string input)
{
    std::string curr = "";
    static unsigned long out[7 + 2];

    for (char c : input)
    {
        if (c == ',')
        {
            int i = std::stoi(curr);
            out[i]++;

            curr = "";
            continue;
        }

        curr.push_back(c);
    }

    out[std::stoi(curr)]++;

    return &out[0];
}

int main()
{
    std::string input;
    std::ifstream inputFile("input.txt");
    std::getline(inputFile, input);
    unsigned long* fisches = splitString(input);

    unsigned long nextDay[7 + 2];
    for (int i = 0; i < 256; i++)
    {
        for (int i = 0; i < 9; i++)
        {
            if (i == 0)
                nextDay[8] = fisches[i];
            else
                nextDay[i - 1] = fisches[i];
        }

        nextDay[6] += nextDay[8];
        for(int i = 0; i < 9; i++)
            fisches[i] = nextDay[i];
    }
    
    unsigned long out = 0;
    long test = 26984457539;
    std::cout << test << std::endl;

    for(int i = 0; i < 9; i++)
    {
        std::cout << fisches[i] << std::endl;
        out += fisches[i];
    }

    std::cout << out << std::endl;

    return -1;
}
