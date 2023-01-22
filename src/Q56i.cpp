#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> singleNumbers(vector<int> &nums) {
    int ss = 0;
    for (auto num : nums) {
      ss ^= num;
    }
    int d = 1;
    while ((d & ss) == 0) {
      d <<= 1;
    }
    int a = 0, b = 0;
    for (auto num : nums) {
      if (d & num)
        a ^= num;
      else
        b ^= num;
    }
    return vector<int>{a, b};
  }
};