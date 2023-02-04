#include <iostream>
#include <vector>
using namespace std;

// 双指针法，没想到，感觉太巧妙了！
class Solution {
public:
  bool checkInclusion(string s1, string s2) {
    int n = s1.length();
    int m = s2.length();
    if (n > m)
      return false;
    int cnt[26] = {0};
    for (char c : s1) {
      --cnt[c - 'a'];
    }
    int l = 0;
    for (int r = 0; r < m; ++r) {
      int x = s2[r] - 'a';
      ++cnt[x];
      while (cnt[x] > 0) {
        --cnt[s2[l] - 'a'];
        ++l;
      }
      if (r - l + 1 == n)
        return true;
    }
    return false;
  }
};