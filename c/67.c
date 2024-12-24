#include <stdlib.h>
#include <stdio.h>
#include "string.h"
#include "stdbool.h"

char* addBinary(char* a, char* b) {
  char* big = strlen(a) >= strlen(b) ? a : b;
  char* small = strlen(a) < strlen(b) ? a : b;
  bool carry = false;

  int big_len = (int)strlen(big);
  int idx_big = big_len - 1;
  int idx_small = (int)strlen(small) - 1;

  int result_len = big_len + 1;
  int result_idx = result_len - 2; // skip null byte
  char* result = malloc(result_len);

  // initialize the result string
  for (int i = 0; i < result_len; i++) {
    result[i] = 0x0;
  }

  while (idx_big >= 0) {
    char c_big = big[idx_big--];
    char c_small = idx_small >= 0 ? small[idx_small--] : '0';

    char c = c_big + c_small - '0';
    char res;

    switch (c) {
      case '0': // case 1: ('0' + '0') = '0'
        res = carry ? '1' : '0';
        carry = false;
        break;
      case '1': // case 2: ('[0|1]' + '[1|0]') - '0' = '1'
        res = '1';
        break;
      case '2': // case 3: ('1' + '1') - '0' = '2'
        res = carry ? '1' : '0';
        carry = true;
        break;
      default:
        printf("don't know what to do for addition value %c. abort!\n", c);
        exit(1);
        break;
    }

    result[result_idx--] = res;
  }

  // special case - first char is 1 and still need to apply the carry...
  if (carry) {
    if (result[0] == '0') {
      result[0] = '1';
    } else { // '1'
      char* tmp = malloc(result_len + 1);
      tmp[0] = '1';
      tmp[result_len] = 0x0; // terminating null byte
      result[0] = '0';
      strcpy(tmp + 1, result);
      free(result);
      result = tmp;
    }
  }

  return result;
}

int main(int argc, char* argv[]) {
  char* a = argv[1];
  char* b = argv[2];
  char* c = addBinary(a, b);
  printf("result of %s + %s = %s\n", a, b, c);
  return 0;
}
