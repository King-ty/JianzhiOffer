#include <iostream>
#include <vector>
using namespace std;

// Definition for a Node.
class Node {
public:
  int val;
  Node *left;
  Node *right;

  Node() {}

  Node(int _val) {
    val = _val;
    left = NULL;
    right = NULL;
  }

  Node(int _val, Node *_left, Node *_right) {
    val = _val;
    left = _left;
    right = _right;
  }
};

class Solution {
private:
  Node *head, *pre;
  void trans(Node *cur) {
    if (cur == nullptr) {
      return;
    }
    trans(cur->left);
    if (pre != nullptr)
      pre->right = cur;
    else
      head = cur;
    cur->left = pre;
    pre = cur;
    trans(cur->right);
  }

public:
  Node *treeToDoublyList(Node *root) {
    if (root == nullptr)
      return nullptr;
    head = pre = nullptr;
    trans(root);
    head->left = pre;
    pre->right = head;
    return head;
  }
};

int main() {
  Node *a = new Node(4);
  Node *b = new Node(2);
  Node *c = new Node(5);
  Node *d = new Node(1);
  Node *e = new Node(3);
  a->left = b;
  a->right = c;
  b->left = d;
  b->right = e;
  Solution solution;
  Node *ret = solution.treeToDoublyList(a);
  cout << ret->val << endl;
  return 0;
}