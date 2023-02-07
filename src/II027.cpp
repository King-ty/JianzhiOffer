
// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
  ListNode *middleNode(ListNode *head) {
    auto fast = head, slow = head;
    while (fast->next != nullptr && fast->next->next != nullptr) {
      slow = slow->next;
      fast = fast->next->next;
    }
    return slow;
  }

  ListNode *reverseList(ListNode *head) {
    ListNode *ret = nullptr;
    auto p = head;
    while (p != nullptr) {
      auto q = p->next;
      p->next = ret;
      ret = p;
      p = q;
    }
    return ret;
  }

  bool _isPalindrome(ListNode *l1, ListNode *l2) {
    while (l1 && l2) {
      if (l1->val != l2->val) {
        return false;
      }
      l1 = l1->next;
      l2 = l2->next;
    }
    return true;
  }

public:
  bool isPalindrome(ListNode *head) {
    if (head == nullptr)
      return true;
    auto mid = middleNode(head);
    auto head2 = mid->next;
    mid->next = nullptr;
    head2 = reverseList(head2);
    return _isPalindrome(head, head2);
  }
};
