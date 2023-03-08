#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

// 并查集解法，太巧妙了！！！
class Solution {
    int find(vector<int>& fa, vector<double>& w, int x) {
        if (fa[x] != x) {
            int root = find(fa, w, fa[x]);
            w[x] *= w[fa[x]];
            fa[x] = root;
        }
        return fa[x];
    }
    void merge(vector<int>& fa, vector<double>& w, int x, int y, double val) {
        int rootx = find(fa, w, x), rooty = find(fa, w, y);
        if (rootx != rooty) {
            fa[rootx] = rooty;
            w[rootx] = val * w[y] / w[x];
        }
    }

  public:
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values,
                                vector<vector<string>>& queries) {
        int n = values.size(), vars_num = 0;
        // 哈希编号
        unordered_map<string, int> vars;
        for (int i = 0; i < n; ++i) {
            string &x = equations[i][0], &y = equations[i][1];
            if (vars.find(x) == vars.end()) {
                vars[x] = vars_num++;
            }
            if (vars.find(y) == vars.end()) {
                vars[y] = vars_num++;
            }
        }
        vector<int> fa(vars_num);
        vector<double> w(vars_num, 1.0);
        for (int i = 0; i < vars_num; ++i) {
            fa[i] = i;
        }
        for (int i = 0; i < n; ++i) {
            int x = vars[equations[i][0]], y = vars[equations[i][1]];
            double val = values[i];
            merge(fa, w, x, y, val);
        }
        vector<double> ret;
        for (auto& query : queries) {
            double res = -1.0;
            if (vars.find(query[0]) != vars.end() && vars.find(query[1]) != vars.end()) {
                int x = vars[query[0]], y = vars[query[1]];
                int rootx = find(fa, w, x), rooty = find(fa, w, y);
                if (rootx == rooty) {
                    res = w[x] / w[y];
                }
            }
            ret.push_back(res);
        }
        return ret;
    }
};
