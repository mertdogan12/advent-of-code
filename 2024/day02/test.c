#include "day.h"

#include <stdio.h>
#include <stdlib.h>

int main() {
    const int correctOutput = 2;
    FILE* file = fopen("test-input.txt", "r");
    if (file == NULL)
        exit(EXIT_FAILURE);

    int result = part1(file);

    if (result == correctOutput) {
        printf("Correct result: %d\n", result);
    } else {
        printf("Wrong result: %d\n", result);
    }

    fclose(file);
    exit(EXIT_SUCCESS);

    result -1;
}
