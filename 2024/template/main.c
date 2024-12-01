#include "main.h"

#include <stdio.h>
#include <stdlib.h>

int part1(FILE* input) {
    return -1;
}

int part2(FILE* input) {
    return -1;
}

int main() {
    FILE* file = fopen("input.txt", "r");
    if (file == NULL)
        exit(EXIT_FAILURE);

    printf("Output: %d\n", part1(file));

    fclose(file);
    if (file != NULL)
        free(file);
    exit(EXIT_SUCCESS);

    return -1;
}
