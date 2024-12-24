#include <stdio.h>
#include "stdbool.h"

int badV;

bool isBadVersion(int n) {
  return n >= badV;
}

int firstBadVersion(int n) {
  int lower = 1;
  int upper = n;
  int mid = lower + (upper - lower) / 2;
  
  while (lower <= upper) {
    if (isBadVersion(mid)) {
      upper = mid - 1;
    } else {
      lower = mid + 1;
    }
    mid = lower + (upper - lower) / 2;
  }

  if (isBadVersion(mid)) {
    return mid;
  } else {
    return mid - 1;
  }
}

int main(int argc, char* argv[]) {
  badV = 4;
  int n = 6;
  int result = firstBadVersion(n);
  printf("the first bad version is %i (expected %i)\n", result, badV);
  return 0;
}
