#include "iostream"
#include "fstream"
#include "vector"
#include "chrono"
#include "thread"
#include "math.h"

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
    std::string s[2] = {"", ""};
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
                if (s[0].empty() || s[1].empty())
                    continue;

                if (from)
                {
                    line.from[0] = std::stoi(s[0]);
                    line.from[1] = std::stoi(s[1]);
                }

                s[0].clear();
                s[1].clear();
                pos = 0;
                from = false;
                break;
            default:
                s[pos].push_back(c);
                break;
        }
    }

    line.to[0] = std::stoi(s[0]);
    line.to[1] = std::stoi(s[1]);

    return line;
}

int main() 
{
    std::string input;
    std::ifstream inputFile("input.txt");

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

    int fields[x2Max + 1][x1Max + 1];
    for (int i = 0; i < SizeofArray(fields); i++)
        for (int j = 0; j < SizeofArray(fields[i]); j++)
            fields[i][j] = 0;

    for (Line line: lines)
    {
        double dir[2];

        dir[0] = line.to[0] - line.from[0];
        dir[1] = line.to[1] - line.from[1];

        int len;
        len = std::sqrt(pow(dir[0], 2) + pow(dir[1], 2));

        dir[0] = std::round(dir[0] / len);
        dir[1] = std::round(dir[1] / len);

        int x1 = line.from[0], x2 = line.from[1];
        fields[x2][x1]++;
        while(true)
        {
            x1 += dir[0];
            x2 += dir[1];
            fields[x2][x1]++;

            if (line.from[0] == line.to[0])
            {
                if (x2 == line.to[1])
                    break;
            }
            else 
            {
                if (x1 == line.to[0])
                    break;
            }
        }
    }

    int out = 0;

    for (int i = 0; i < SizeofArray(fields); i++)
        for (int j = 0; j < SizeofArray(fields[i]); j++)
                if (fields[i][j] > 1)
                    out++;

    Log("Sum: " << out);

    return 1;
}
