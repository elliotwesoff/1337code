#include <stdio.h>

int search_iterative(int* nums, int numsSize, int target) {
  int left = 0;
  int right = numsSize - 1;

  while (left <= right) {
    int i = (left + right) / 2; // floor by design
    int mid = nums[i];

    if (target > mid) {
      left = i + 1;
    } else if (target < mid) {
      right = i - 1;
    } else {
      return i;
    }
  }

  return -1;
}

int search_recursive(int* nums, int left, int right, int target) {
  if (left > right)
    return -1;

  int i = (left + right) / 2; // floor by design
  int mid = nums[i];

  if (mid == target)
    return i;

  if (target > mid) {
    left = i + 1;
  } else {
    right = i - 1;
  }

  return search_recursive(nums, left, right, target);
}

int search(int* nums, int numsSize, int target) {
  return search_recursive(nums, 0, numsSize - 1, target);
}

int main() {
  int nums[] = { -1,0,3,5,9,12 };
  int target = 9;
  int result = search(nums, 6, target);
  printf("the index of %i in nums is: %i\n", target, result);
  return 0;
}
