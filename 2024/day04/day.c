#include "day.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Vec2 {
    int x;
    int y;
} vec2;

int match(const vec2 start, const vec2 direction, char** input, const int height, const int width) {
    const char* search = "XMAS";
    vec2 pos = start;

    printf("\n");
    for(int i = 0; i < 4; i++) {
        printf("\n");
        printf("%d %d | ", direction.x, direction.y);
        printf("%d %d | ", pos.x, pos.y);

        if (pos.x < 0 || pos.y < 0)
            return 0;

        if (pos.x >= width || pos.y >= height)
            return 0;

        printf("%c | ", input[pos.y][pos.x]);

        if (input[pos.y][pos.x] != search[i])
            return 0;

        pos.x += direction.x;
        pos.y += direction.y;
    }

    printf("yes\n");
    return 1;
}

int part(FILE* input) {
    char *line;
    size_t len = 0;
    ssize_t read;
    ssize_t width = -2;
    size_t height = 0;
    char* buffer[1000];

    while ((read = getline(&line, &len, input)) != -1) {
      char* cursor = line;

      buffer[height] = malloc(read);
      memcpy(buffer[height], cursor, read);
      height++;

      if (width == -2)
          width = read;
    }

    int out = 0;
    width--;
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            vec2 start = {x, y};

            if (buffer[y][x] != 'X')
                continue;

            for (int h = -1; h <= 1; h++)
                for (int w = -1; w <= 1; w++) {
                    vec2 dir = {w, h};

                    if (match(start, dir, buffer, height, width)) {
                        out++;
                    }
                }
        }

        printf("\n-----------\n");
    }

    return out;
}
