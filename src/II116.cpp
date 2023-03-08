#include <iostream>
#include <queue>
#include <vector>

using namespace std;

// BFS
class Solution {
  public:
    int findCircleNum(vector<vector<int>>& isConnected) {
        int n = isConnected.size();
        vector<bool> vis(n);
        auto bfs = [&vis, n, &isConnected](int i) {
            queue<int> qq;
            qq.emplace(i);
            vis[i] = true;
            while (!qq.empty()) {
                int cur = qq.front();
                qq.pop();
                for (int j = 0; j < n; ++j) {
                    if (isConnected[cur][j] && !vis[j]) {
                        vis[j] = true;
                        qq.emplace(j);
                    }
                }
            }
        };
        int res = 0;
        for (int i = 0; i < n; ++i) {
            if (!vis[i]) {
                bfs(i);
                ++res;
            }
        }
        return res;
    }
};
