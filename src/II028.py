"""
# Definition for a Node.
"""


class Node:
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child


class Solution:
    def flatten(self, head: 'Node') -> 'Node' | None:
        ret = Node(0, None, None, None)
        cur = ret
        st = [head]
        while st:
            p = st.pop()
            while p is not None:
                cur.next = p
                p.prev = cur
                cur = p
                p = p.next
                if cur.child is not None:
                    st.append(p)
                    p = cur.child
                    cur.child = None
        cur.next = None
        ret = ret.next
        if ret is not None:
            ret.prev = None
        return ret
