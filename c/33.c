#include <stdio.h>
#include <assert.h>

// Given the array nums after the possible rotation and an integer target,
// return the index of target if it is in nums, or -1 if it is not in nums.

#define min(a, b) (a < b ? a : b)
#define max(a, b) (a > b ? a : b)

int search(int* nums, int numsSize, int target) {
    int ctr = 0;
    int len = numsSize;
    int arrLeft = 0;
    int arrRight = numsSize - 1;
    int leftStart, leftEnd, rightStart, mid;
    int rightEnd = arrRight;

    while (++ctr <= numsSize) {
      if (len == 1) {
        return nums[arrLeft] == target ? arrLeft : -1;
      } else if (len == 2) {
        if (nums[arrLeft] == target) {
          return arrLeft;
        } else if (nums[arrRight] == target) {
          return arrRight;
        } else {
          return -1;
        }
      }

      mid = arrLeft + ((arrRight - arrLeft) / 2);

      leftStart = arrLeft;
      leftEnd = mid;
      rightStart = min(mid + 1, rightEnd);
      rightEnd = arrRight;

      if ((nums[leftStart] <= nums[leftEnd] && target >= nums[leftStart] && target <= nums[leftEnd])
          || (nums[leftStart] >= nums[leftEnd] && (target >= nums[leftStart] || target <= nums[leftEnd]))) {
        arrLeft = leftStart;
        arrRight = leftEnd;
      }

      else if ((nums[rightStart] <= nums[rightEnd] && target >= nums[rightStart] && target <= nums[rightEnd])
          || (nums[rightStart] >= nums[rightEnd] && (target >= nums[rightStart] || target <= nums[rightEnd]))) {
        arrLeft = rightStart;
        arrRight = rightEnd;
      }

      len = max(arrRight - arrLeft + 1, 1);
    }

    return -1;
}

int main() {
  int nums[7] = { 4,5,6,7,0,1,2 };
  int nums2[1] = { 1 };
  int nums3[7] = { 6,7,1,2,3,4,5 };
  int nums4[3] = { 1,3,5 };
  int nums5[5] = { 5,1,2,3,4 };
  int nums6[8] = { 4,5,6,7,8,1,2,3 };
  int nums7[9] = { 2,3,4,5,6,7,8,9,1 };

  printf("target index: %i\n", search(nums, 7, 0)); // 4
  printf("target index: %i\n", search(nums, 7, 3)); // -1
  printf("target index: %i\n", search(nums2, 1, 0)); // -1
  printf("target index: %i\n", search(nums2, 1, 1)); // 0
  printf("target index: %i\n", search(nums3, 7, 6)); // 0
  printf("target index: %i\n", search(nums4, 3, 5)); // 2
  printf("target index: %i\n", search(nums5, 5, 1)); // 1
  printf("target index: %i\n", search(nums6, 8, 8)); // 4
  printf("target index: %i\n", search(nums7, 9, 9)); // 7
  return 0;
}
