#include <iostream>
#include <numeric>
#include <unordered_map>
#include <vector>

using namespace std;

// 极其暴力使用hash的解法
class Solution {
public:
  int findTargetSumWays(vector<int>& nums, int target) {
    int n = nums.size(), m = accumulate(nums.begin(), nums.end(), 0);
    vector<unordered_map<int, int>> dp(n + 1);
    dp[0][0] = 1;
    for (int i = 0; i < n; ++i) {
      for (int j = -m; j <= m; ++j) {
        dp[i + 1][j] = 0;
        if (dp[i].count(j - nums[i])) {
          dp[i + 1][j] += dp[i][j - nums[i]];
        }
        if (dp[i].count(j + nums[i])) {
          dp[i + 1][j] += dp[i][j + nums[i]];
        }
      }
    }
    return dp[n].count(target) ? dp[n][target] : 0;
  }
};
