#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
  ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
    unordered_set<ListNode *> us;
    while (headA != nullptr && headB != nullptr) {
      if (us.count(headA)) {
        return headA;
      }
      if (us.count(headB)) {
        return headB;
      }
      if (headA == headB) {
        return headA;
      }
      us.insert(headA);
      us.insert(headB);
      headA = headA->next;
      headB = headB->next;
    }
    while (headA != nullptr) {
      if (us.count(headA)) {
        return headA;
      }
      headA = headA->next;
    }
    while (headB != nullptr) {
      if (us.count(headB)) {
        return headB;
      }
      headB = headB->next;
    }
    return nullptr;
  }
};