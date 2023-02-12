public class II049 {

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
    int getRes(TreeNode node, int product) {
        if (node == null)
            return 0;
        if (node.left == null && node.right == null)
            return product * 10 + node.val;
        return getRes(node.left, product * 10 + node.val) + getRes(node.right, product * 10 + node.val);
    }

    public int sumNumbers(TreeNode root) {
        return getRes(root, 0);
    }
}
