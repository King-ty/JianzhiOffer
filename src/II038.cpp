#include <iostream>
#include <stack>
#include <utility>
#include <vector>

using namespace std;

class Solution {
public:
  vector<int> dailyTemperatures(vector<int> &temperatures) {
    stack<pair<int, int>> st;
    vector<int> ret(temperatures.size());
    int len = temperatures.size();
    for (int i = 0; i < len; ++i) {
      while (!st.empty() && st.top().first < temperatures[i]) {
        auto nw = st.top();
        st.pop();
        ret[nw.second] = i - nw.second;
      }
      st.push({temperatures[i], i});
    }
    return ret;
  }
};
