# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def reverseList(self, head: ListNode) -> ListNode | None:
        ret = None
        p = head
        while p is not None:
            q = p.next
            p.next = ret
            ret = p
            p = q
        return ret
