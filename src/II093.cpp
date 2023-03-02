#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
  int lenLongestFibSubseq(vector<int>& arr) {
    unordered_map<int, int> map;
    int n = arr.size();
    for (int i = 0; i < n; ++i) {
      map.emplace(arr[i], i);
    }
    vector<vector<int>> f(n, vector(n, 2));
    int res = 0;
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n && arr[j] < arr[i] * 2; ++j) {
        if (map.count(arr[j] - arr[i])) {
          int k = map[arr[j] - arr[i]];
          f[i][j] = f[k][i] + 1;
          res = max(res, f[i][j]);
        }
      }
    }
    return res;
  }
};

int main() {
  Solution solu;
  vector<int> arr{2, 4, 5, 6, 7, 8, 11, 13, 14, 15, 21, 22, 34};
  cout << solu.lenLongestFibSubseq(arr) << endl;
}

// class Solution {
//   vector<int> f;
//   int getLongest(int ind, vector<int>& arr) {
//     int l = 0, r = ind - 1, res = 0;
//     while (l < r) {
//       if (arr[l] + arr[r] < arr[ind]) {
//         ++l;
//       } else if (arr[l] + arr[r] > arr[ind]) {
//         --r;
//       } else {
//         res = max(res, f[l] + 2);
//         ++l;
//         --r;
//       }
//     }
//     return res;
//   }

// public:
//   int lenLongestFibSubseq(vector<int>& arr) {
//     int n = arr.size(), res = 1;
//     f = vector(n, 1);

//     for (int i = 2; i < n; ++i) {
//       f[i] = getLongest(i, arr);
//       res = max(res, f[i]);
//     }
//     return res;
//   }
// };
