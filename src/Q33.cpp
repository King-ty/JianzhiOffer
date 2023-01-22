#include <iostream>
#include <stack>
#include <vector>

using namespace std;

class Solution {
public:
  bool verifyPostorder(vector<int> &postorder) {
    stack<int> st;
    int root = INT_MAX;
    for (auto it = postorder.rbegin(); it != postorder.rend(); ++it) {
      if (*it > root)
        return false;
      while (!st.empty() && st.top() > *it) {
        root = st.top();
        st.pop();
      }
      st.push(*it);
    }
    return true;
  }
};