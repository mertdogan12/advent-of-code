#include "iostream"
#include "fstream"
#include "string"

int main() 
{
    int prev = -1;
    int now;
    int count = 0;
    std::string input;

    std::ifstream InputFile("input-day1.txt");
    
    while (getline(InputFile, input)) {
        now = std::stoi(input);
        std::cout << now << std::endl;

        if (prev < now && prev != -1) 
            count++;

        prev = now;
    }

    std::cout << "\nCount: " << count << std::endl;

    return 1;
}
