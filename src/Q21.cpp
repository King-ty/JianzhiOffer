#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> exchange(vector<int> &nums) {
    vector<int> odds, evens;
    for (auto num : nums) {
      if (num & 1) {
        odds.emplace_back(num);
      } else {
        evens.emplace_back(num);
      }
    }
    odds.insert(odds.end(), evens.begin(), evens.end());
    return odds;
  }
};