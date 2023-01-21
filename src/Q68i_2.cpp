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
public:
  TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
    auto cur = root;
    for (;;) {
      if (p->val < cur->val && q->val < cur->val) {
        cur = cur->left;
      } else if (p->val > cur->val && q->val > cur->val) {
        cur = cur->right;
      } else {
        break;
      }
    }
    return cur;
  }
};