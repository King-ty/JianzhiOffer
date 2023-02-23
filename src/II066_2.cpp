#include <array>
#include <cstddef>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

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
