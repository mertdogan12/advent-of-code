#include "iostream"
#include "fstream"
#include "vector"
#include "string"

std::vector<int> splitString(std::string input)
{
    std::string curr = "";
    std::vector<int> out;

    for (char c : input)
    {
        if (c == ',')
        {
            int i = std::stoi(curr);
            out.push_back(i);

            curr = "";
            continue;
        }

        curr.push_back(c);
    }

    out.push_back(std::stoi(curr));

    return out;
}

template<typename T>
void logVec(std::vector<T> input)
{
    std::string out = "";

    for(int i = 0; i < input.size(); i++)
    {
        std::cout << input.at(i);
        if (i != input.size() - 1)
            std::cout << ", ";
    }

    std::cout << std::endl;
}

int main()
{
    std::string input;
    std::ifstream inputFile("input.txt");
    std::getline(inputFile, input);
    std::vector<int> fisches = splitString(input);

    std::cout << "Initial state: ";
    logVec<int>(fisches);

    for (int i = 0; i < 80; i++)
    {
        std::vector<int> nextDay;
        for (int fisch : fisches)
        {
            if (fisch == 0)
            {
                nextDay.push_back(6);
                nextDay.push_back(6 + 2);
            } else
                nextDay.push_back(fisch - 1);
        }

        fisches = nextDay;
        nextDay.clear();

        std::cout << "Day " << i + 1 << ": ";
        logVec<int>(fisches);
    }
    
    std::cout << "Fisches: " << fisches.size() << std::endl;

    return -1;
}
