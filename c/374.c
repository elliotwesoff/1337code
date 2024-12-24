#include <stdlib.h>
#include <stdio.h>

int pick;

int guess(int num) {
  if (num < pick)
    return 1;

  if (num > pick)
    return -1;

  return 0;
}

int guessNumber(int n) {
  int result = -1;
  int max = 0x7fffffff;
  int mid = max / 2;
  int lower = 0;
  int upper = max; // positive int max

  while (1) {
    result = guess(n);
    switch (result) {
      case 0:
        return n;
      case 1: // n < PICK
        lower = n + 1;
        break;
      case -1:
        upper = n - 1;
        break;
      default:
        exit(1);
    } 

    n = lower + ((upper - lower) / 2);
  }

  return n;
}

int main(int argc, char* argv[]) {
  int n;
  n = 2126753390;
  pick = 1702766719;
  int result = guessNumber(n);
  printf("pick is %i, number is %i\n", pick, result);
  return 0;
}
