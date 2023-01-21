// 该方法其实对所有树都有效，不局限于二叉搜索树
#include <iostream>
#include <vector>
using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
private:
  TreeNode *ans;
  int find(TreeNode *root, TreeNode *p, TreeNode *q) {
    int ret = 0;
    if (root == nullptr)
      return 0;
    ret += (root == p) + (root == q) + find(root->left, p, q) +
           find(root->right, p, q);
    if (ret == 2 && ans == nullptr) {
      ans = root;
    }
    return ret;
  }

public:
  TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
    ans = nullptr;
    find(root, p, q);
    return ans;
  }
};