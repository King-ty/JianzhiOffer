//  Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

function detectCycle(head: ListNode | null): ListNode | null {
    let vs: Set<ListNode> = new Set();
    for (let node = head; node != null; node = node.next) {
        if (vs.has(node)) {
            return node;
        }
        vs.add(node);
    }
    return null;
};

export default detectCycle;
