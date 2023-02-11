import java.util.LinkedList;
import java.util.Queue;

public class II045 {

}

// Definition for a binary tree node.
class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {
    }

    TreeNode(int val) {
        this.val = val;
    }

    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

class Solution {
    public int findBottomLeftValue(TreeNode root) {
        Queue<TreeNode> qq = new LinkedList<>();
        qq.offer(root);
        int ret = 0;
        while (!qq.isEmpty()) {
            var cur = qq.poll();
            ret = cur.val;
            if (cur.right != null) {
                qq.offer(cur.right);
            }
            if (cur.left != null) {
                qq.offer(cur.left);
            }
        }
        return ret;
    }
}
