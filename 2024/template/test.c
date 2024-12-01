#include "main.h"

#include <stdio.h>
#include <stdlib.h>

int main() {
    const int correctOutput = 0;
    FILE* file = fopen("input.txt", "r");
    if (file == NULL)
        exit(EXIT_FAILURE);

    int result = part1(file);

    if (result == correctOutput) {
        printf("Correct result: %d", result);
    } else {
        printf("Wrong result: %d", result);
    }

    fclose(file);
    if (file != NULL)
        free(file);
    exit(EXIT_SUCCESS);

    result -1;
}
