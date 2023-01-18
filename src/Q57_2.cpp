#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    vector<int> ret;
    for (int l = 0, r = nums.size() - 1; l < r;) {
      if (nums[l] + nums[r] == target) {
        ret.push_back(nums[l]);
        ret.push_back(nums[r]);
        break;
      } else if (nums[l] + nums[r] < target) {
        l++;
      } else {
        r--;
      }
    }
    return ret;
  }
};