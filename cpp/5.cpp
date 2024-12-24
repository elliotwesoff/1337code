#include <iostream>
#include <string>

class Solution {
public:
  std::string longestPalindrome(std::string s) {
       std::string longestpal = "";
       volatile int largest = 0, len = s.length();

       for (int i = 0; i < len; i++) {
         for (int j = len - 1; j >= i; j--) {
           if (j - i >= largest && check_pal(i, j, s)) {
             volatile int pallength = j - i + 1;
             longestpal = s.substr(i, pallength);
             largest = pallength;
           }
         }
       }
       return longestpal;
    }

  bool check_pal(int i, int j, std::string &s) {
    while (i <= j) {
      if (s[i] != s[j]) {
        return false;
      }
      i++;
      j--;
    }
    return true;
  }
};

