#include "day.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum creaseType {
	increase = 1,
	decrease = -1,
	unclear = 0,
};

int part1(FILE* input) {
	char* line = NULL;
	size_t len = 0;
	ssize_t read;
	int out = 0;
	
	while ((read = getline(&line, &len, input)) != -1) {
		int i = atoi(strtok(line, " "));
		char* j;
		int safe = 1;
		enum creaseType type = unclear;

		while ((j = strtok(NULL, " ")) != NULL) {
			int k = atoi(j);

			switch(type) {
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

int part2(FILE* input) {

}
