#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
  public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> exists;
        int n = nums.size();
        for (int i = 0; i < n; ++i) {
            int num = nums[i];
            if (exists.find(target - num) != exists.end()) {
                return vector<int>{exists[target - num], i};
            }
            exists.emplace(num, i);
        }
        return vector<int>();
    }
};
