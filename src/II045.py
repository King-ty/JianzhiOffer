# Definition for a binary tree node.
from queue import Queue


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def findBottomLeftValue(self, root: TreeNode) -> int:
        qq = Queue()
        qq.put((root, 1))
        dep = 0
        ret = 0
        while not qq.empty():
            (cur, cur_dep) = qq.get()
            if cur_dep > dep:
                dep = cur_dep
                ret = cur.val
            if cur.left:
                qq.put((cur.left, cur_dep+1))
            if cur.right:
                qq.put((cur.right, cur_dep+1))
        return ret
