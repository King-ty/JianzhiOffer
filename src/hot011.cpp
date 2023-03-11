#include <iostream>
#include <vector>
using namespace std;

// 双指针，很巧妙的思路
class Solution {
  public:
    int maxArea(vector<int>& height) {
        int l = 0, r = height.size() - 1, res = 0;
        while (l < r) {
            if (height[l] < height[r]) {
                res = max(res, height[l] * (r - l));
                ++l;
            } else {
                res = max(res, height[r] * (r - l));
                --r;
            }
        }
        return res;
    }
};
