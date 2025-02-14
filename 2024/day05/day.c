#include "day.h"

#include "set.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct vec2 
{
    int x;
    int y;
} vec2;

// https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key
size_t hash(size_t x) 
{
    x = ((x >> 16) ^ x) * 0x45d9f3b;
    x = ((x >> 16) ^ x) * 0x45d9f3b;
    x = (x >> 16) ^ x;
    return x;
}

size_t hash_vec2(void* value) 
{
    vec2* xy = (vec2*) value;
    size_t hash_x = hash(xy->x);
    size_t hash_y = hash(xy->y);

    return (hash_x + 1) * (hash_y - 8);
}

int check(Set* set, int* array, size_t arrSize)
{
    for (size_t i = 1; i < arrSize; i++)
    {
        vec2 val;
        val.x = array[i];
        val.y = array[0];
        printf("%d|%d\n", val.x, val.y);

        if (exists(set, (void*) &val))
        {
            return 0;
        }
    }
    
    return 1;
}

int part(FILE* input) 
{
    Set* set = create_set(hash_vec2);

    char* line;
    size_t len = 0;
    ssize_t read;
    int first = 1;
    int out = 0;
    
    while ((read = getline(&line, &len, input)) != -1)
    {
        if (first)
        {
            if (!strcmp(line, "\n"))
            {
                first = 0;
                continue;
            }

            vec2* inp = malloc(sizeof(vec2));
            inp->x = atoi(strtok(line, "|"));
            inp->y = atoi(strtok(NULL, "|"));

            add(set, inp);
        } else
        {
            int buffInx = 0;
            int buffer[1000];

            buffer[buffInx] = atoi(strtok(line, ","));
            buffInx++;

            char* num;
            while ((num = strtok(NULL, ",")) != NULL)
            {
                buffer[buffInx] = atoi(num);
                buffInx++;
            }
            
            int correct = 1;
            printf("Size: %d\n", buffInx);
            for (size_t i = 0; i < buffInx - 1; i++)
            {
                printf("First: %d\n", buffer[i]);
                if (!check(set, &buffer[i], buffInx - i))
                {
                    correct = 0;
                    printf("nope\n");
                    break;
                } else
                {
                    printf("yes: \n");
                }
            }

            if (correct)
            {
                out += buffer[(buffInx / 2 + 1) - 1];
                printf("value: %d\n", buffer[(buffInx / 2 + 1) - 1]);
            }
            
            printf("\n");
        }
    }

    vec2 t;
    t.x = 72;
    t.y = 44;
    printf("%d\n", exists(set, (void*) &t));
    return out;
}