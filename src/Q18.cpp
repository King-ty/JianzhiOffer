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
    ListNode **cur = &head;
    while (*cur != nullptr) {
      if ((*cur)->val == val) {
        *cur = (*cur)->next;
        break;
      }
      cur = &((*cur)->next);
    }
    return head;
  }
};