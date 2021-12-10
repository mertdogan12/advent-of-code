#include "iostream"
#include "fstream"
#include "string"
#include "vector"

int main() 
{
    std::vector<int> elems;
    int prev = -1;
    int now;
    int count = 0;
    std::string input;

    std::ifstream InputFile("input.txt");

    while (getline(InputFile, input)) 
    {
        elems.push_back(std::stoi(input));
    }
    
    for (int i = 0; i < elems.size() - 2; i++) {
        now = elems.at(i) + elems.at(i + 1) + elems.at(i + 2);

        if (prev < now && prev != -1) 
            count++;

        prev = now;
    }

    std::cout << "\nCount: " << count << std::endl;

    return 1;
}
