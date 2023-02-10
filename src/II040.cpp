#include <iostream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

using namespace std;

class Solution {
  vector<int> cnt;
  int getLineRes() {
    stack<int> st;
    st.push(-1);
    cnt.push_back(0);
    int n = cnt.size(), res = 0;
    for (int i = 0; i < n; ++i) {
      while (st.size() > 1 && cnt[st.top()] > cnt[i]) {
        int height = cnt[st.top()];
        st.pop();
        int width = i - st.top() - 1;
        res = max(res, height * width);
      }
      st.push(i);
    }

    cnt.pop_back();
    return res;
  }

public:
  int maximalRectangle(vector<string> &matrix) {
    if (matrix.empty())
      return 0;
    int n = matrix[0].size(), res = 0;
    cnt = vector<int>(n);
    for (auto line : matrix) {
      for (int i = 0; i < n; ++i) {
        cnt[i] = line[i] == '0' ? 0 : cnt[i] + 1;
      }
      res = max(res, getLineRes());
    }
    return res;
  }
};

int main() {
  Solution solu;
  vector<string> test{"10100", "10111", "11111", "10010"};
  cout << solu.maximalRectangle(test) << endl;

  return 0;
}
