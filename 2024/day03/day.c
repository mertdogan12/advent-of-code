#include "day.h"

#include <regex.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *match(const char *input, int *output, int* enabled) {
  regex_t regex;
  regmatch_t match[3];
  char errorBuffer[100];
  int rc;
  char *cursor = input;

  if ((rc = regcomp(&regex, "mul\\(([0-9]+),([0-9]+)\\)|do\\(\\)|don't\\(\\)", REG_EXTENDED)) != 0) {
    regerror(rc, &regex, errorBuffer, 100);
    printf("Could not compile regex: %s\n", errorBuffer);
    return NULL;
  }

  if ((rc = regexec(&regex, cursor, 3, match, 0)) == 0) {
    printf("Found: %zd\n", regex.re_nsub);
    int offset = 0;
    int x, y;

    for (int i = 0; i <= regex.re_nsub; i++) {
      regoff_t startOff = match[i].rm_so;
      regoff_t endOff = match[i].rm_eo;
      regoff_t diff = endOff - startOff;

      char group[diff + 1];
      strncpy(group, cursor + startOff, diff);
      group[diff] = '\0';
      printf("Group %d (%d): %s\n", i, diff, group);

      switch (i) {
      case 0:
        offset = endOff;
        break;
      case 1:
        x = atoi(group);
        break;
      case 2:
        y = atoi(group);
        break;
      }

      if (strcmp(group, "don't()") == 0) {
        *enabled = 0;
        break;
      }

      if (strcmp(group, "do()") == 0) {
        *enabled = 1;
        x = 0;
        y = 0;
        break;
      }
    }

    if (*enabled) {
        *output += x * y;
        printf("%d\n", *output);
    }
    cursor += offset;
  } else {
    regerror(rc, &regex, errorBuffer, 100);
    printf("Could not exec regex: %s\n", errorBuffer);
    return NULL;
  }

  return cursor;
}

int part(FILE *input) {
  char *line;
  size_t len = 0;
  ssize_t read;
  int output = 0;
  int enabled = 1;

  while ((read = getline(&line, &len, input)) != -1) {
    char* cursor = line;

    while((cursor = match(cursor, &output, &enabled)) != NULL);
  }

  return output;
}
