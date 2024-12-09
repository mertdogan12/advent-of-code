#include "day.h"

#include <stdio.h>
#include <stdlib.h>

int main() {
    FILE* file = fopen("input.txt", "r");
    if (file == NULL)
        exit(EXIT_FAILURE);

    printf("Output: %d\n", part2(file));

    fclose(file);
    exit(EXIT_SUCCESS);

    return -1;
}
