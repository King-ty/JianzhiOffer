#include <iostream>
#include <vector>
using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

// 复用l1
class Solution {
  public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int c = 0;
        auto p1 = l1, p2 = l2;
        ListNode* pre = nullptr;
        for (; p1 != nullptr && p2 != nullptr; p1 = p1->next, p2 = p2->next) {
            p1->val += p2->val + c;
            c = p1->val / 10;
            p1->val %= 10;
            pre = p1;
        }
        if (p2 != nullptr) {
            pre->next = p2;
            p1 = p2;
        }
        for (; c > 0 && p1 != nullptr; p1 = p1->next) {
            p1->val += c;
            c = p1->val / 10;
            p1->val %= 10;
            pre = p1;
        }
        if (c > 0) {
            pre->next = new ListNode(c);
        }
        return l1;
    }
};
