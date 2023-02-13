
// Definition for a binary tree node.
class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function increasingBST(root: TreeNode | null): TreeNode | null {
  let ret: TreeNode | null = null, cur: TreeNode | null = null;
  function dfs(node: TreeNode | null) {
    if (node === null) return;
    dfs(node.left);
    node.left = null;
    if (cur === null) {
      ret = cur = node;
    } else {
      cur.right = node;
      cur = node;
    }
    dfs(node.right);
  }
  dfs(root);
  if (cur !== null)
    (cur as TreeNode).right = null;
  return ret;
};

export default increasingBST;
