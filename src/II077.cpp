#include <iostream>
#include <utility>
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

// 自下而上链表归并排序
class Solution {
  static pair<ListNode*, ListNode*> merge(ListNode* head1, ListNode* head2) {
    ListNode dummyHead = ListNode(0);
    auto p = &dummyHead;
    while (head1 && head2) {
      if (head1->val < head2->val) {
        p->next = head1;
        head1 = head1->next;
      } else {
        p->next = head2;
        head2 = head2->next;
      }
      p = p->next;
    }
    if (head1) {
      p->next = head1;
    } else if (head2) {
      p->next = head2;
    }
    while (p->next) p = p->next;
    return {dummyHead.next, p};
  }

public:
  ListNode* sortList(ListNode* head) {
    int len = 0;
    for (auto p = head; p != nullptr; p = p->next) {
      ++len;
    }
    ListNode dummyHead(0, head);
    for (int sub_len = 1; sub_len < len; sub_len <<= 1) {
      auto prev = &dummyHead, p = dummyHead.next;
      while (p != nullptr) {
        auto head1 = p;
        for (int i = 1; i < sub_len && p->next != nullptr;
             ++i, p = p->next) { // 注意从1开始！因为p已经指向第1个节点了
        }
        auto head2 = p->next;
        p->next = nullptr;
        p = head2;
        for (int i = 1; i < sub_len && p != nullptr && p->next != nullptr;
             ++i, p = p->next) { // 注意从1开始！
        }
        ListNode* next = nullptr;
        if (p != nullptr) {
          next = p->next;
          p->next = nullptr;
        }
        auto [first, last] = merge(head1, head2);
        prev->next = first;
        // last->next = next;
        prev = last;
        p = next;
      }
    }
    return dummyHead.next;
  }
};

int main() {
  auto list = new ListNode(-1, new ListNode(5, new ListNode(3, new ListNode(4, new ListNode(0)))));
  Solution solu;
  solu.sortList(list);
}
