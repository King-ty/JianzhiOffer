#include <iostream>
#include <queue>
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

class CBTInserter {
  TreeNode *root;
  vector<TreeNode *> nodes;

public:
  CBTInserter(TreeNode *root) {
    this->root = root;
    queue<TreeNode *> nodeQ;
    if (root)
      nodeQ.push(root);
    while (!nodeQ.empty()) {
      auto cur = nodeQ.front();
      nodeQ.pop();
      if (cur->left) {
        nodeQ.push(cur->left);
      }
      if (cur->right) {
        nodeQ.push(cur->right);
      }
      nodes.push_back(cur);
    }
  }

  int insert(int v) {
    auto newNode = new TreeNode(v);
    nodes.push_back(newNode);
    int curNum = nodes.size();
    if (curNum == 1) {
      root = newNode;
      return 0;
    }
    auto fa = nodes[curNum / 2 - 1];
    if (curNum & 1) {
      fa->right = newNode;
    } else {
      fa->left = newNode;
    }
    return fa->val;
  }

  TreeNode *get_root() { return root; }
};

/**
 * Your CBTInserter object will be instantiated and called as such:
 * CBTInserter* obj = new CBTInserter(root);
 * int param_1 = obj->insert(v);
 * TreeNode* param_2 = obj->get_root();
 */
