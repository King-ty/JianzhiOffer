#include <array>
#include <functional>
#include <iostream>
#include <numeric>
#include <string>
#include <unordered_map>
#include <vcruntime.h>
#include <vector>

using namespace std;

// 这题思路其实不难的，但是因为对于C++11一些特性不熟悉，导致部分参考了答案的写法
class Solution {
public:
  vector<vector<string>> groupAnagrams(vector<string> &strs) {
    auto myHash = [fn = hash<int>{}](const array<int, 26> &arr) -> size_t {
      return accumulate(arr.begin(), arr.end(), 0U, [&](size_t acc, int num) {
        return (acc << 1) ^ fn(num);
      });
    };
    unordered_map<array<int, 26>, int, decltype(myHash)> mp(0, myHash);
    vector<vector<string>> ret;
    for (auto str : strs) {
      array<int, 26> cnts{};
      for (auto c : str) {
        cnts[c - 'a']++;
      }
      if (!mp.count(cnts)) {
        mp.insert({cnts, ret.size()});
        ret.push_back(vector<string>{str});
      } else {
        ret[mp.at(cnts)].push_back(str);
      }
    }
    return ret;
  }
};

int main() {
  vector<string> temp{"eat", "tea", "tan", "ate", "nat", "bat"};
  Solution solu;
  solu.groupAnagrams(temp);

  return 0;
}
