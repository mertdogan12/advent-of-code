#include "iostream"
#include "fstream"
#include "vector"
#include "array"
#include "math.h"

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

int main()
{
    // [cout of 0, cout of 1]
    std::string input;
    std::ifstream InputFile("input.txt");
    std::vector<std::array<int, 2>> vh;
    bool first = true;

    while (getline(InputFile, input))
    {
        for (int i = 0; i < input.length(); i++) 
        {
            char c = input.at(i);

            if (first)
            {
                vh.push_back({0, 0});
            }
            
            if (c == '0')
                vh.at(i)[0]++;
            else if (c == '1')
                vh.at(i)[1]++;
        }
        
        first = false;
    }

    std::string grString;
    std::string erString;

    for(int i = 0; i < vh.size(); i++)
    {
        std::array<int, 2> arr = vh.at(i);

        if (arr[0] > arr[1]) 
        {
            grString += '0';
            erString += '1';
        }
        else if (arr[0] < arr[1])
        {
            grString += '1';
            erString += '0';
        }
    }

    std::cout << grString << " " << erString << std::endl;
    std::cout << binToInt(grString) * binToInt(erString) << std::endl;


    return 1;
}
