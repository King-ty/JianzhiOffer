#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  int lengthOfLongestSubstring(string s) {
    int cnt[128] = {0};
    int l = 0, len = s.length(), ret = 0;
    for (int r = 0; r < len; ++r) {
      int x = s[r];
      ++cnt[x];
      while (cnt[x] > 1) {
        int y = s[l];
        --cnt[y];
        ++l;
      }
      ret = max(ret, r - l + 1);
    }
    return ret;
  }
};
