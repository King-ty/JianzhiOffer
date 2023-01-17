#include <vector>

using namespace std;
class Solution {
public:
  int maxProfit(vector<int> &prices) {
    int min_price = 1e9, ret = 0;
    for (auto price : prices) {
      ret = max(ret, price - min_price);
      min_price = min(min_price, price);
    }
    return ret;
  }
};