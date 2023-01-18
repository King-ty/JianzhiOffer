#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    vector<int> ret;
    for (auto l = nums.begin(), r = nums.end() - 1; l < r;) {
      if (*l + *r == target) {
        ret.push_back(*l);
        ret.push_back(*r);
        break;
      } else if (*l + *r < target) {
        l++;
      } else {
        r--;
      }
    }
    return ret;
  }
};