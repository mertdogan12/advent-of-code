#include "iostream"
#include "fstream"
#include "vector"

#define Log(x) std::cout << x << std::endl;
#define LogVec(x) for (std::string elem: x) std::cout << elem << ' '; std::cout << std::endl;
#define LogVecInt(x) for (int elem: x) std::cout << elem << ' '; std::cout << std::endl;
#define SizeofArray(arr) (sizeof(arr) / sizeof(*arr))

struct Line 
{
    int from[2];
    int to[2];

    void LogLine() 
    {
        std::cout << from[0] << ", " << from[1] << " --> " << to[0] << ", " << to[1] << std::endl;
    }
};

Line getInputLine(std::string input)
{
    Line line;
    int pos = 0;
    bool from = true;

    for(char c: input)
    {
        switch(c)
        {
            case ',': 
                pos++;
                break;
            case ' ': case '-': case '>':
                pos = 0;
                from = false;
                break;
            default:
                if (from)
                    line.from[pos] = c - '0';
                else
                    line.to[pos] = c - '0';

                break;
        }
    }

    return line;
}

int main() 
{
    std::string input;
    std::ifstream inputFile("test.txt");

    std::vector<Line> lines;
    int x1Max = 0;
    int x2Max = 0;

    while (std::getline(inputFile, input))
    {
        Line line = getInputLine(input);
        lines.push_back(line);

        if (line.from[0] > x1Max)
            x1Max = line.from[0];

        if (line.to[0] > x1Max)
            x1Max = line.to[0];

        if (line.from[1] > x2Max)
            x2Max = line.from[1];

        if (line.to[1] > x2Max)
            x2Max = line.to[1];
    }

    int fields[x1Max][x2Max];
    for (int i = 0; i < SizeofArray(fields); i++)
        for (int j = 0; j < SizeofArray(fields[i]); j++)
            fields[i][j] = 0;

    for (int i = 0; i < SizeofArray(fields); i++)
    {
        for (int j = 0; j < SizeofArray(fields[i]); j++)
            std::cout << fields[i][j];

        std::cout << std::endl;
    }


    return 1;
}
