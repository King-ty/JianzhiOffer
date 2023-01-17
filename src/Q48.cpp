#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  int lengthOfLongestSubstring(string s) {
    vector<int> pos = vector(128, -1);
    int l = 0, r = 0, ret = 0;
    int len = s.length();
    for (int i = 0; i < len; ++i) {
      int c = s[i];
      r += 1;
      if (pos[c] == -1) {
        ret = max(ret, r - l);
      } else {
        for (; l <= pos[c]; ++l) {
          // cout << s[l] << endl;
          pos[s[l]] = -1;
        }
      }
      pos[c] = i;
    }
    return ret;
  }
};

int main() {
  string s = "tmmzuxt";
  // string s = "bbtablud";
  Solution a;
  cout << a.lengthOfLongestSubstring(s) << endl;
  return 0;
}