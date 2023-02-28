#include <algorithm>
#include <iostream>
#include <type_traits>
#include <vector>
using namespace std;

class Solution {
  vector<string> cur;
  vector<vector<string>> res;

  bool is_pa_string(string& s) {
    for (int i = 0, j = s.size() - 1; i < j; ++i, --j) {
      if (s[i] != s[j]) return false;
    }
    return true;
  }

  void get_res(const string& s, int i_start = 0) {
    int n = s.size();
    if (i_start >= n) {
      res.push_back(cur);
    }
    for (int i = 1; i <= n - i_start; ++i) {
      auto sub_s = s.substr(i_start, i);
      if (is_pa_string(sub_s)) {
        cur.push_back(sub_s);
        get_res(s, i_start + i);
        cur.pop_back();
      }
    }
  }

public:
  vector<vector<string>> partition(string s) {
    get_res(s);
    return res;
  }
};

int main() {
  Solution solu;
  solu.partition("google");
}
