#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int* plusOne(int* digits, int digitsSize, int* returnSize) {
    // allocate the returned array. if there's a carry all
    // the way to the leftmost digit, we'll need an extra
    // digit place in the array.

    bool carry = false; // end carry

    // determine end carry. slight performance hit of 2N
    for (int i = digitsSize - 1; i >= 0; i--) {
      if (digits[i] == 9 && i == 0) {
        carry = true;
        break;
      }
      if (digits[i] == 9) {
        continue;
      } else {
        break;
      }
    }

    int returnedSize = (digitsSize * sizeof(int)) + (carry ? sizeof(int) : 0);
    int* returned = malloc(returnedSize);
    *returnSize = returnedSize / sizeof(int);

    if (carry) {
        returned[0] = 1;
    }

    bool addDone = false;
    
    // starting from the back, try to add 1
    // for (int i = digitsSize - 1; i >= 0; i--) {
    for (int i = digitsSize - 1, retIdx = i + carry; i >= 0; i--, retIdx--) {
        int current = digits[i];
        int retIdx = i + carry;

        if (addDone) {
            returned[retIdx] = current; // + 1 done, just copy the rest
            continue;
        }        

        if (current == 9) {
            returned[retIdx] = 0;
            continue; // continue the carry
        }

        returned[retIdx] = current + 1;
        addDone = true;
    }

    return returned;
}

int main() {
  // int digits[] = { 9, 9, 9 };
  int digits[] = { 9,8,7,6,5,4,3,2,1,0 };
  // int digits[] = { 9 };
  int digitsSize = 10;
  int* returnSize = malloc(sizeof(int));

  int* returned = plusOne(digits, digitsSize, returnSize);

  for (int i = 0; i < *returnSize; i++) {
    printf("%i ", returned[i]);
  }

  printf("\n");

  free(returnSize);

  return 0;
}
