// 树状数组法
#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

class Bit {
private:
  vector<int> tree;
  int n;

public:
  Bit(int _n) : tree(_n, 0), n(_n) {}

  int lowbit(int x) { return x & (-x); }

  void update(int x) {
    while (x < n) {
      tree[x] += 1;
      x += lowbit(x);
    }
  }

  int query(int x) {
    int ret = 0;
    while (x) {
      ret += tree[x];
      x -= lowbit(x);
    }
    return ret;
  }
};

class Solution {
public:
  int reversePairs(vector<int> &nums) {
    // 离散化
    vector<int> temp(nums);
    sort(temp.begin(), temp.end());
    for (auto &num : nums) {
      num = lower_bound(temp.begin(), temp.end(), num) - temp.begin() +
            1; // +1是因为树状数组从1开始
    }
    Bit bit(nums.size());
    int ret = 0;
    for (auto it = nums.rbegin(); it != nums.rend(); ++it) {
      ret += bit.query(*it - 1);
      bit.update(*it);
    }
    return ret;
  }
};