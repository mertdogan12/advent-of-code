#include "day.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum creaseType {
  increase = 1,
  decrease = -1,
  unclear = 0,
};

void worng(int* safe, int* skip, const int i, const int k) {
    (*safe)--;
}

int testInput(const int x, const int y, enum creaseType* type) {
      switch (*type) {
      case increase:
        if (x > y) {
          return 0;
        }
        break;

      case decrease:
        if (x < y) {
          return 0;
        }
        break;

      case unclear:
        if (x > y) {
          *type = decrease;
        } else if (x < y) {
          *type = increase;
        }
        break;
      }

      int diff = y - x;
      if (diff < 0)
        diff *= -1;

      if (diff < 1 || diff > 3) {
        return 0;
      }

      printf("test success %d %d\n", x, y);

      return 1;
}

int testInputs(const int* input, const int size, const int excludedIndex) {
    enum creaseType type = unclear;
    printf("Test: %d\n", excludedIndex);

    int s = size - 1;
    if (excludedIndex == s)
        s--;

    for(int i = 0; i < s; i++) {
        int nextI = i + 1;
        if (nextI == excludedIndex) 
            nextI++;

        int cur = input[i];
        int next = input[nextI];

        if (excludedIndex != -1 && excludedIndex == i)
            continue;

        if (!testInput(cur, next, &type)) {
            printf("Test failed: %d\n", i);
            return i;
        }
    }

    printf("Test success\n");
    return -1;
}

int part1(FILE *input) {
  char *line = NULL;
  size_t len = 0;
  ssize_t read;
  int out = 0;

  while ((read = getline(&line, &len, input)) != -1) {
    int i = atoi(strtok(line, " "));
    char *j;
    int safe = 1;
    enum creaseType type = unclear;

    while ((j = strtok(NULL, " ")) != NULL) {
      int k = atoi(j);

      switch (type) {
      case increase:
        if (k < i) {
          safe = 0;
        }
        break;
      case decrease:
        if (k > i) {
          safe = 0;
        }
        break;
      case unclear:
        if (k > i) {
          type = increase;
        } else if (k < i) {
          type = decrease;
        }
        break;
      }

      int diff = k - i;
      if (diff < 0)
        diff *= -1;

      if (safe == 0 || diff < 1 || diff > 3) {
        safe = 0;
        printf("Wrong: %d %d\n", i, k);
        break;
      }

      i = k;
    }

    if (safe) {
      out++;
      printf("%s\n", line);
    }
  }

  return out;
}

int part2(FILE *input) {
  char *line = NULL;
  size_t len = 0;
  ssize_t read;
  int out = 0;
  int j, k, l;

  while ((read = getline(&line, &len, input)) != -1) {
    int buf[50];
    char *c;
    int safe = 1;
    int i = 1;

    buf[0] = atoi(strtok(line, " "));

    while ((c = strtok(NULL, " ")) != NULL) {
      int cur = atoi(c);
      buf[i] = cur;
      i++;
    }

    if ((j = testInputs(buf, i, -1)) != -1) {
        if (testInputs(buf, i, j) != -1 && testInputs(buf, i, j - 1) != -1 && testInputs(buf, i, j + 1) != -1) {
            safe = 0;
        }
    }

    if (safe) {
      out++;
      printf("Safe\n");
    }
    printf("\n");
  }

  return out;
}
