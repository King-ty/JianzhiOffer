#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

/*
// Definition for a Node.
*/
class Node {
public:
  int val;
  Node *next;
  Node *random;

  Node(int _val) {
    val = _val;
    next = NULL;
    random = NULL;
  }
};

/**
 * O(n)思路，使用umap记录指向关系
 */
class Solution {
private:
  // 从原链表指针到新建节点指针的映射
  unordered_map<Node *, Node *> umap;

public:
  Node *copyRandomList(Node *head) {
    if (head == nullptr) {
      return nullptr;
    }
    if (umap.count(head)) {
      return umap.at(head);
    } else {
      auto ret = new Node(head->val);
      umap.insert({head, ret});
      ret->next = copyRandomList(head->next);
      ret->random = copyRandomList(head->random);
      return ret;
    }
  }
};

int main() { return 0; }