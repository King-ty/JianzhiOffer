import java.util.ArrayList;
import java.util.List;

public class II044 {

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
    void dfs(TreeNode node, List<Integer> ret, int dep) {
        if (node == null)
            return;
        if (dep == ret.size()) {
            ret.add(node.val);
        } else {
            ret.set(dep, Math.max(ret.get(dep), node.val));
        }
        dfs(node.left, ret, dep + 1);
        dfs(node.right, ret, dep + 1);
    }

    public List<Integer> largestValues(TreeNode root) {
        List<Integer> ret = new ArrayList<>();
        dfs(root, ret, 0);
        return ret;
    }
}
