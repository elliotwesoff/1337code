#include <stdio.h>

//  O(log(n)) strategy:
//
//  1. handle edge cases:
//      i. if size == 1, return the value
//      ii. if size == 2, return the min value
//      ii. if first < last, the array is sorted/not-rotated,
//          return first value
//  2. cut array in half. find minimum value in first and
//     last values of *both* arrays. new array becomes the
//     one with the min.
//  3. go back to 1 using array from 2

#define min(a, b) (a < b ? a : b)
#define max(a, b) (a > b ? a : b)

int findMin(int* nums, int numsSize) {
    int i = 0, j = numsSize - 1; // current array bounds
    int size = numsSize;
    int mid, minLeft, minRight;
    int a, b, c, d; // left and right array bounds

    while (1) {
        // 1.
        if (size == 1) {
            return nums[i];
        } else if (size == 2) {
            return min(nums[i], nums[j]);
        } else if (nums[i] < nums[j]) {
            return nums[i];
        }

        // 2.
        mid = (i + j) / 2;
        a = i;
        b = max(mid - 1, 0);
        c = mid;
        d = j;

        minLeft = min(nums[a], nums[b]);
        minRight = min(nums[c], nums[d]);

        // 3.
        if (minLeft < minRight) {
            i = a;
            j = b;
        } else {
            i = c;
            j = d;
        }

        size = j - i + 1;
    }
}

int main() {
  // int nums[5] = { 3,4,5,1,2 }; int numsSize = 5;
  int nums[2] = { 2, 1 }; int numsSize = 2;
  int min = findMin(nums, numsSize);
  printf("min value: %i\n", min);
  return 0;
}
