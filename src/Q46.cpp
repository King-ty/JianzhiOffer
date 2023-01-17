#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
  int translateNum(int num) {
    auto num_str = to_string(num);
    int len = num_str.length();
    vector<int> f = vector(len + 1, 0);
    f[0] = 1;
    for (int i = 1; i <= len; ++i) {
      f[i] += f[i - 1];
      if (i > 1) {
        auto sub = num_str.substr(i - 2, 2);
        if (sub >= "10" && sub < "26") {
          f[i] += f[i - 2];
        }
      }
    }
    return f[len];
  }
};