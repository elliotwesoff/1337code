#include <stdio.h>
#include <stdlib.h>
#include "stdbool.h"

bool search(int* nums, int size, int target) {
  int left = 0;
  int right = size - 1;

  while (left <= right) {
    int i = (left + right) / 2;
    int mid = nums[i];

    if (target > mid) {
      left = i + 1;
    } else if (target < mid) {
      right = i - 1;
    } else {
      return true;
    }
  }

  return false;
}

bool searchMatrix(int** matrix, int matrixSize, int* matrixColSize, int target) {
  // int row_idx = 0;
  // int row_val = matrix[row_idx][0];
  //
  // while (row_val <= target && ++row_idx < matrixSize) {
  //   row_val = matrix[row_idx][0];
  // }
  //
  // if (row_idx > matrixSize)
  //   return false;
  //
  // int* row = matrix[row_idx - (row_idx != 0)];

  int* row;
  int lower = 0;
  int upper = matrixSize - 1;

  // calculate the middle index and value
  // if target >= mid, we know it can't be in the left half,
  // so move the left boundary up to mid's index.
  // if target < mid, we know it can't be in the right half,
  // so move the right boundary down to mid's index - 1.
  // if left bound >= right bound, look in the row at the left
  // boundary position.

  while (lower < upper) {
    // calculate mid
    int i = (lower + upper) / 2;
    int mid = matrix[i][0];

    if (target > mid) {
      lower = i + 1;
    } else if (target < mid) {
      upper = i - 1;
    } else {
      return true;
    }
  }

  row = matrix[lower];

  // debug
  printf("target %i should be somewhere in here:\n", target);
  for (int i = 0; i < *matrixColSize; i++) {
    printf("%i ", row[i]);
  }
  printf("\n");

  return search(row, *matrixColSize, target);
}

int main(int argc, char* argv[]) {
  int matrix[3][4] = { { 1, 3, 5, 7 }, { 10, 11, 16, 20 }, { 23, 30, 34, 60 } };
  int** m = (int**)malloc(3 * sizeof(int*));

  for (int i = 0; i < 3; i++) {
    m[i] = (int*)malloc(4 * sizeof(int));
    if (m[i] == NULL) {
      perror("Failed to allocate memory");
      return 1;
    }
    for (int j = 0; j < 4; j++) {
      m[i][j] = matrix[i][j];
    }
  }

  int mColSize = 4;
  int target = atoi(argv[1]);
  bool result = searchMatrix(m, 3, &mColSize, target);
  printf("target %i was %s found\n", target, result ? "" : "not");
  return 0;
}
