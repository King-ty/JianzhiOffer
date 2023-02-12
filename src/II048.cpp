#include <cstddef>
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Codec {
public:
  // Encodes a tree to a single string.
  string serialize(TreeNode *root) {
    queue<TreeNode *> qq;
    qq.push(root);
    string ret;
    while (!qq.empty()) {
      TreeNode *cur = qq.front();
      qq.pop();
      if (cur == nullptr) {
        ret += ",null";
      } else {
        ret += "," + to_string(cur->val);
        qq.push(cur->left);
        qq.push(cur->right);
      }
    }
    ret[0] = '[';
    return ret + "]";
  }

  // Decodes your encoded data to tree.
  TreeNode *deserialize(string data) {
    if (data == "[]")
      return nullptr;
    queue<TreeNode *> qq;
    int index = 1;

    while (index < int(data.length())) {
      int end = getValue(data, index);
      if (data.substr(index, end - index) == "null") {
        qq.push(nullptr);
      } else {
        int val = atoi(data.substr(index, end - index).c_str());
        qq.push(new TreeNode(val));
      }
      index = end + 1;
    }

    TreeNode *ret = nullptr;
    queue<TreeNode *> qq2;
    qq2.push(qq.front());
    qq.pop();
    while (!qq2.empty()) {
      TreeNode *cur = qq2.front();
      qq2.pop();
      if (ret == nullptr)
        ret = cur;
      if (cur == nullptr)
        continue;
      if (!qq.empty()) {
        cur->left = qq.front();
        qq2.push(qq.front());
        qq.pop();
      } else {
        cur->left = nullptr;
        qq2.push(nullptr);
      }
      if (!qq.empty()) {
        cur->right = qq.front();
        qq2.push(qq.front());
        qq.pop();
      } else {
        cur->right = nullptr;
        qq2.push(nullptr);
      }
    }
    return ret;
  }

private:
  int getValue(const string &data, int i) {
    for (; i < int(data.length()); ++i) {
      if (data[i] == ',' || data[i] == ']')
        break;
    }
    return i;
  }
};

// Your Codec object will be instantiated and called as such:
// Codec codec;
// codec.deserialize(codec.serialize(root));