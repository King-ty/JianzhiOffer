#include <functional>
#include <iostream>
#include <numeric>
#include <queue>
#include <vector>

using namespace std;

// BFS解法
// 使用二维数组，无辅助空间
class Solution {
    static constexpr int MOV[][2]{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

  public:
    vector<vector<int>> updateMatrix(vector<vector<int>>& mat) {
        queue<pair<int, int>> qq;
        int m = mat.size(), n = mat[0].size();
        vector<vector<int>> res(m, vector<int>(n, -1));
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (mat[i][j] == 0) {
                    qq.emplace(i, j);
                    res[i][j] = 0;
                }
            }
        }
        while (!qq.empty()) {
            auto [x, y] = qq.front();
            qq.pop();
            for (auto& mv : MOV) {
                int xx = x + mv[0], yy = y + mv[1];
                if (xx >= 0 && xx < m && yy >= 0 && yy < n && res[xx][yy] == -1) {
                    res[xx][yy] = res[x][y] + 1;
                    qq.emplace(xx, yy);
                }
            }
        }
        return res;
    }
};

// 使用unordered_map
// class Solution {
//   public:
//     vector<vector<int>> updateMatrix(vector<vector<int>>& mat) {
//         const int MOV[][2]{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
//         auto my_hash = [fn = hash<int>{}](const pair<int, int>& pii) -> size_t {
//             return (fn(pii.first) << 1) ^ fn(pii.second);
//         };
//         queue<pair<int, int>> qq;
//         unordered_map<pair<int, int>, int, decltype(my_hash)> vis(0, my_hash);
//         int m = mat.size(), n = mat[0].size();
//         vector<vector<int>> res(m, vector<int>(n));
//         for (int i = 0; i < m; ++i) {
//             for (int j = 0; j < n; ++j) {
//                 if (mat[i][j] == 0) {
//                     qq.emplace(i, j);
//                     vis.insert({{i, j}, 0});
//                 }
//             }
//         }
//         while (!qq.empty()) {
//             auto [x, y] = qq.front();
//             qq.pop();
//             int dis = vis[{x, y}];
//             res[x][y] = dis;
//             for (auto& mv : MOV) {
//                 if (x + mv[0] >= 0 && x + mv[0] < m && y + mv[1] >= 0 && y + mv[1] < n
//                     && !vis.count({x + mv[0], y + mv[1]})) {
//                     vis.insert({{x + mv[0], y + mv[1]}, dis + 1});
//                     qq.emplace(x + mv[0], y + mv[1]);
//                 }
//             }
//         }
//         return res;
//     }
// };

int main() {
    Solution solu;
    vector<vector<int>> vv{{0}, {0}, {0}, {0}, {0}};
    solu.updateMatrix(vv);
}
