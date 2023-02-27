#include <iostream>
#include <vector>
using namespace std;

// 说好了搜索与回溯，题解怎么是迭代
class Solution {
public:
  vector<vector<int>> subsets(vector<int>& nums) {
    vector<int> cur;
    vector<vector<int>> res;
    int n = nums.size();
    for (int mask = 0; mask < 1 << n; ++mask) {
      cur.clear();
      for (int i = 0; i < n; ++i) {
        if (mask & (1 << i)) {
          cur.push_back(nums[i]);
        }
      }
      res.push_back(cur);
    }
    return res;
  }
};
