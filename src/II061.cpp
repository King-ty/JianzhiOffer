#include <iostream>
#include <queue>
#include <utility>
#include <vector>

using namespace std;

// 想知道push和emplace的性能差距
class Solution {
public:
  vector<vector<int>> kSmallestPairs(vector<int>& nums1, vector<int>& nums2, int k) {
    auto cmp = [&nums1, &nums2](auto& p1, auto& p2) {
      return nums1[p1.first] + nums2[p1.second] > nums1[p2.first] + nums2[p2.second];
    };
    int m = nums1.size(), n = nums2.size();
    priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(cmp)> pq(cmp);
    for (int i = 0; i < m; ++i) {
      // pq.emplace(i, 0);
      pq.push({i, 0});
    }
    vector<vector<int>> ret;
    while (k-- && !pq.empty()) {
      auto [x, y] = pq.top();
      pq.pop();
      // ret.emplace_back(initializer_list<int>{nums1[x], nums2[y]});
      ret.push_back({nums1[x], nums2[y]});
      if (y < n - 1) {
        // pq.emplace(x, y + 1);
        pq.push({x, y + 1});
      }
    }
    return ret;
  }
};
