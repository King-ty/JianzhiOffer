#include <iostream>
#include <vector>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
  public:
  TreeNode* inorderSuccessor(TreeNode* root, TreeNode* p) {
    TreeNode* ret = nullptr;
    while (root != nullptr) {
      if (root->val > p->val) {
        ret = root;
        root = root->left;
      } else {
        root = root->right;
      }
    }
    return ret;
  }
};
