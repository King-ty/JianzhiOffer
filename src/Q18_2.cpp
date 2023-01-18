#include <iostream>
#include <vector>
using namespace std;

/**
 * Definition for singly-linked list.
 */
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
class Solution {
public:
  ListNode *deleteNode(ListNode *head, int val) {
    ListNode *cur = head, *pre = nullptr;
    while (cur != nullptr) {
      if (cur->val == val) {
        if (pre == nullptr) {
          return cur->next;
        } else {
          pre->next = cur->next;
          break;
        }
      }
      pre = cur;
      cur = cur->next;
    }
    return head;
  }
};