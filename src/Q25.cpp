#include <cstddef>
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
  ListNode *mergeTwoLists(ListNode *l1, ListNode *l2) {
    ListNode ret = ListNode(0), *cur = &ret;
    while (l1 != nullptr && l2 != nullptr) {
      if (l1->val < l2->val) {
        cur->next = l1;
        l1 = l1->next;
      } else {
        cur->next = l2;
        l2 = l2->next;
      }
      cur = cur->next;
      cur->next = nullptr;
    }
    if (l1 != nullptr) {
      cur->next = l1;
    } else {
      cur->next = l2;
    }
    return ret.next;
  }
};