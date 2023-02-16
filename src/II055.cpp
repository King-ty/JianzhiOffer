#include <iostream>
#include <stack>
#include <vector>
using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class BSTIterator {
  TreeNode *cur;
  stack<TreeNode *> st;

public:
  BSTIterator(TreeNode *root) { cur = root; }

  int next() {
    if (hasNext()) {
      while (cur) {
        st.push(cur);
        cur = cur->left;
      }
      auto temp = st.top();
      st.pop();
      cur = temp->right;
      int ret = temp->val;
      delete temp;
      return ret;
    }
    return -1;
  }

  bool hasNext() { return !st.empty() || cur; }
};

/**
 * Your BSTIterator object will be instantiated and called as such:
 * BSTIterator* obj = new BSTIterator(root);
 * int param_1 = obj->next();
 * bool param_2 = obj->hasNext();
 */
