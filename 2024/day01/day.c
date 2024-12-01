#include "day.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node {
    int value;
    struct node* next;
} node_t;

int compare(const void* a, const void* b) {
    int int_a = *((int *) a);
    int int_b = *((int *) b);

    if (int_a == int_b)
        return 0;
    if (int_a < int_b)
        return -1;

    return 1;
}

int part1(FILE* input) {
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    node_t* first_l = (node_t*) malloc(sizeof(node_t));
    node_t* first_r = (node_t*) malloc(sizeof(node_t));

    node_t* current_l = (node_t*) malloc(sizeof(node_t));
    node_t* current_r = (node_t*) malloc(sizeof(node_t));

    int count = 0;

    while((read = getline(&line, &len, input)) != -1) {
        int left = atoi(strtok(line, " "));
        int right = atoi(strtok(NULL, " "));
        printf("read: %d %d\n", left, right);

        if (count == 0) {
            first_l->value = left;
            first_r->value = right;
        } else if (count == 1) {
            current_l->value = left;
            current_r->value = right;

            first_l->next = current_l;
            first_r->next = current_r;
        } else {
            current_l->next = (node_t*) malloc(sizeof(node_t));
            current_r->next = (node_t*) malloc(sizeof(node_t));

            current_l->next->value = left;
            current_r->next->value = right;

            current_l = current_l->next;
            current_r = current_r->next;
        }

        count++;
    }

    int leftValues[count];
    int rightValues[count];
    node_t* iter_l = first_l;
    node_t* iter_r = first_r;

    for(int i = 0; i < count; i++) {
        leftValues[i] = iter_l->value;
        rightValues[i] = iter_r->value;

        iter_r = iter_r->next;
        iter_l = iter_l->next;
    }

    qsort(leftValues, count, sizeof(int), compare);
    qsort(rightValues, count, sizeof(int), compare);

    int out = 0;
    for(int i = 0; i < count; i++) {
        printf("%d %d\n", leftValues[i], rightValues[i]);
        int diff = rightValues[i] - leftValues[i];

        if (diff < 0)
            diff *= -1;

        out += diff;
    }
    printf("\n");

    if (line != NULL)
        free(line);

    return out;
}

int part2(FILE* input) {
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    node_t* first_l = (node_t*) malloc(sizeof(node_t));
    node_t* first_r = (node_t*) malloc(sizeof(node_t));

    node_t* current_l = (node_t*) malloc(sizeof(node_t));
    node_t* current_r = (node_t*) malloc(sizeof(node_t));

    int count = 0;

    while((read = getline(&line, &len, input)) != -1) {
        int left = atoi(strtok(line, " "));
        int right = atoi(strtok(NULL, " "));
        printf("read: %d %d\n", left, right);

        if (count == 0) {
            first_l->value = left;
            first_r->value = right;
        } else if (count == 1) {
            current_l->value = left;
            current_r->value = right;

            first_l->next = current_l;
            first_r->next = current_r;
        } else {
            current_l->next = (node_t*) malloc(sizeof(node_t));
            current_r->next = (node_t*) malloc(sizeof(node_t));

            current_l->next->value = left;
            current_r->next->value = right;

            current_l = current_l->next;
            current_r = current_r->next;
        }

        count++;
    }

    int leftValues[count];
    int rightValues[count];
    node_t* iter_l = first_l;
    node_t* iter_r = first_r;

    for(int i = 0; i < count; i++) {
        leftValues[i] = iter_l->value;
        rightValues[i] = iter_r->value;

        iter_r = iter_r->next;
        iter_l = iter_l->next;
    }

    qsort(leftValues, count, sizeof(int), compare);
    qsort(rightValues, count, sizeof(int), compare);

    int out = 0;
    int lowerI = 0;
    int lowerValue = -1;
    for(int i = 0; i < count; i++) {
        int l = leftValues[i];
        int app = 0;

        for(int j = 0; j < count; j++) {
            int r = rightValues[j];

            if (r > l) 
                break;

            if (r == l)
                app++;
        }

        out += l * app;
    }
    printf("\n");

    if (line != NULL)
        free(line);

    return out;
}
