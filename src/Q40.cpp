#include <iostream>
#include <vector>
using namespace std;

class Solution {
private:
  void partition(vector<int> &arr, int l, int r, int k) {
    int ll = l, rr = r;
    int i = rand() % (r - l + 1) + l;
    swap(arr[i], arr[l]);
    int cur = arr[l];
    while (l < r) {
      while (l < r && arr[r] >= cur) {
        --r;
      }
      arr[l] = arr[r];
      while (l < r && arr[l] <= cur) {
        ++l;
      }
      arr[r] = arr[l];
    }
    arr[l] = cur;
    if (l == k || l == k - 1) {
      return;
    } else if (l < k - 1) {
      partition(arr, r + 1, rr, k);
    } else {
      partition(arr, ll, r - 1, k);
    }
  }

public:
  vector<int> getLeastNumbers(vector<int> &arr, int k) {
    vector<int> ret;
    if (k == 0)
      return ret;
    partition(arr, 0, arr.size() - 1, k);
    ret.insert(ret.end(), arr.begin(), arr.begin() + k);
    return ret;
  }
};

int main() {
  vector<int> a;
  int b[4] = {3, 2, 1, 1};
  a.insert(a.end(), b, b + 3);
  Solution solu;
  auto c = solu.getLeastNumbers(a, 2);
  for (auto it : c) {
    cout << it << ' ';
  }
  return 0;
}

// [0,0,2,3,2,1,1,2,0,4]
// 10

// [0,1,1,2,4,4,1,3,3,2]
// 6