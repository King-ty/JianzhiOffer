#include <iostream>
#include <vector>
using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

using ll = long long;
class Solution {
  ll res;
  ll getRes(TreeNode *node) {
    if (node == nullptr)
      return 0;
    ll lval = max(getRes(node->left), 0LL),
       rval = max(getRes(node->right), 0LL);
    res = max(res, lval + rval + node->val);
    return max(lval, rval) + node->val;
  }

public:
  int maxPathSum(TreeNode *root) {
    res = root->val;
    getRes(root);
    return res;
  }
};
