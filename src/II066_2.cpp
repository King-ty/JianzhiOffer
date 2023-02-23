#include <array>
#include <cstddef>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

// 听说暴力法很厉害，实测不如字典树法
class MapSum {
  unordered_map<string, int> cnt;

public:
  /** Initialize your data structure here. */
  MapSum() {}

  void insert(string key, int val) { cnt[key] = val; }

  int sum(string prefix) {
    int res = 0;
    for (auto &[key, val] : cnt) {
      if (key.substr(0, prefix.length()) == prefix) res += val;
    }
    return res;
  }
};
