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

function rightSideView(root: TreeNode | null): number[] {
    if (root === null) return [];
    let ret: number[] = [];
    let qq: [TreeNode, number][] = [];
    qq.push([root, 1]);
    let dep = 0;
    while (qq.length) {
        let [cur, cur_dep] = qq.shift() as [TreeNode, number];
        if (cur_dep > dep) {
            dep = cur_dep;
            ret.push(cur.val);
        }
        if (cur.right) {
            qq.push([cur.right, cur_dep + 1]);
        }
        if (cur.left) {
            qq.push([cur.left, cur_dep + 1]);
        }
    }
    return ret;
};
