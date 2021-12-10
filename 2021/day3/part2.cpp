#include "iostream"
#include "fstream"
#include "math.h"
#include "vector"

#define LogVec(x) for (std::string elem: x) std::cout << elem << ' ' ; std::cout << std::endl;

int binToInt(std::string inp)
{
    int out = 0;

    for (int i = inp.length() - 1; i >= 0; i--)
    {
        if (inp.at(i) == '1')
        {
            int vact = 2;

            vact = pow(vact, -(i - (inp.length() - 1)));

            out += vact;
        }
    }

    return out;
}

char mostCommonAt(const std::vector<std::string> input, int pos)
{
    int one = 0;
    int zero = 0;

    for (int i = 0; i < input.size(); i++)
    {
        if (input.at(i).at(pos) == '1')
            one++;
        else if (input.at(i).at(pos) == '0')
            zero++;
    }

    return (zero > one) ? '0' : '1';
}

std::string getBits(std::vector<std::string> input, bool most)
{
    for (int i = 0; i < input.at(0).length(); i++)
    {
        std::vector<std::string> left;
        char common = mostCommonAt(input, i);
        if (!most)
            common = (common == '0') ? '1' : '0';

        for (int j = 0; j < input.size(); j++)
        {
            if (input.at(j).at(i) == common)
                left.push_back(input.at(j));
        }

        input = left;

        if (input.size() == 1)
        {
            return input.at(0);
        }
    }

    return input.at(0);
}

int main()
{
    // [cout of 0, cout of 1]
    std::string input;
    std::ifstream InputFile("input.txt");
    std::vector<std::string> elements;

    while (getline(InputFile, input))
    {
        elements.push_back(input);
    }

    std::cout << binToInt(getBits(elements, true)) * binToInt(getBits(elements, false)) << std::endl;

    return 1;
}
