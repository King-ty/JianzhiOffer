#include <iostream>
#include <unordered_map>
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
  unordered_map<int, vector<int>::iterator> mp;
  vector<int>::iterator ith;
  TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder,
                      vector<int>::iterator l, vector<int>::iterator r) {
    if (l >= r)
      return nullptr;
    auto node = new TreeNode(*ith);
    auto i = mp.at(*ith);
    ith++;
    node->left = buildTree(preorder, inorder, l, i);
    node->right = buildTree(preorder, inorder, i + 1, r);
    return node;
  }

public:
  TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder) {
    for (auto it = inorder.begin(); it != inorder.end(); ++it) {
      mp.insert({*it, it});
    }
    ith = preorder.begin();
    return buildTree(preorder, inorder, inorder.begin(), inorder.end());
  }
};

int main() {
  int a[10] = {3, 9, 20, 15, 7}, b[10] = {9, 3, 15, 20, 7};
  vector<int> aa, bb;
  aa.insert(aa.end(), a, a + 5);
  bb.insert(bb.end(), b, b + 5);
  Solution solu;
  solu.buildTree(aa, bb);

  cout << "OK" << endl;

  return 0;
}
