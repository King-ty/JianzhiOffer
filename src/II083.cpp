#include <iostream>
#include <vector>
using namespace std;

// 基于交换的回溯法
class Solution {
  void getRes(vector<int>& nums, vector<vector<int>>& res, int i_start = 0) {
    int n = nums.size();
    if (i_start == n) {
      res.push_back(nums);
    }
    for (int i = i_start; i < n; ++i) {
      swap(nums[i_start], nums[i]);
      getRes(nums, res, i_start + 1);
      swap(nums[i_start], nums[i]);
    }
  }

public:
  vector<vector<int>> permute(vector<int>& nums) {
    vector<vector<int>> res;
    getRes(nums, res);
    return res;
  }
};
