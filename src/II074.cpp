#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
  vector<vector<int>> merge(vector<vector<int>>& intervals) {
    sort(intervals.begin(), intervals.end());
    // [](auto a, auto b) {
    //   if (a[1] != b[1]) return a[1] < b[1];
    //   return a[0] < b[0];
    // };
    vector<vector<int>> ret{intervals[0]};
    auto cur = ret.end() - 1;
    for (auto it = intervals.begin() + 1; it != intervals.end(); ++it) {
      if ((*it)[0] > (*cur)[1]) {
        ret.emplace_back(*it);
        cur = ret.end() - 1;
      } else if ((*it)[1] > (*cur)[1]) {
        (*cur)[1] = (*it)[1];
      }
    }
    return ret;
  }
};
