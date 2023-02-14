# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def inorderSuccessor(self, root: 'TreeNode', p: 'TreeNode') -> 'TreeNode' | None:
        st: list[TreeNode] = []
        cur = root
        prev = None
        while st or cur:
            while cur:
                st.append(cur)
                cur = cur.left
            cur = st.pop()
            if prev == p:
                return cur
            prev = cur
            cur = cur.right
        return None
