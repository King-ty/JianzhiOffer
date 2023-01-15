#include <iostream>
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
 * O(n^2)思路，先建普通链表，建完后对于每个random，O(n)遍历确定位置并建立指向关系。
 * 未完成
 */
class Solution {
public:
  Node *copyRandomList(Node *head) {
    auto ret = head;
    Node *temp = NULL;
    for (auto node = head; node != NULL; node = node->next) {
      if (temp == NULL) {
        temp = new Node(node->val);
      } else {
        temp->next = new Node(node->val);
        temp = temp->next;
        temp = temp->next;
      }
    }
    return ret;
  }
};

int main() { return 0; }