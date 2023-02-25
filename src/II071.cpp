#include <algorithm>
#include <iostream>
#include <iterator>
#include <numeric>
#include <random>
#include <vector>

using namespace std;

// 传统方法
// class Solution {
//   vector<int> nums;
//   int tot;

// public:
//   Solution(vector<int>& w) : nums(std::move(w)) {
//     for (auto it = nums.begin() + 1; it != nums.end(); ++it) {
//       *it += *(it - 1);
//     };
//     tot = nums[nums.size() - 1];
//   }

//   int pickIndex() { return upper_bound(nums.begin(), nums.end(), rand() % tot) - nums.begin(); }
// };

// 题解做法，其实算了2遍求和，会有时间浪费
// class Solution {
//   vector<int> nums;
//   mt19937 gen;
//   uniform_int_distribution<int> dis;

// public:
//   Solution(vector<int>& w) : gen(random_device{}()), dis(1, accumulate(w.begin(), w.end(), 0)) {
//     partial_sum(w.begin(), w.end(), back_inserter(nums));
//   }

//   int pickIndex() {
//     int x = dis(gen);
//     return lower_bound(nums.begin(), nums.end(), x) - nums.begin();
//   }
// };

class Solution {
  vector<int> nums;
  mt19937 gen;
  uniform_int_distribution<int> dis;

public:
  Solution(vector<int>& w) : nums(std::move(w)), gen(random_device{}()) {
    for (auto it = nums.begin() + 1; it != nums.end(); ++it) {
      *it += *(it - 1);
    };
    int tot = nums[nums.size() - 1];
    dis = uniform_int_distribution<int>(1, tot);
  }

  int pickIndex() {
    int x = dis(gen);
    return lower_bound(nums.begin(), nums.end(), x) - nums.begin();
  }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(w);
 * int param_1 = obj->pickIndex();
 */
