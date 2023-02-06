//  Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

// 快慢指针法，比较巧妙！
function detectCycle(head: ListNode | null): ListNode | null {
    let fast = head, slow: ListNode | null | undefined = head;
    while (fast !== null) {
        if (fast.next === null || slow === null)
            return null;
        fast = fast.next.next;
        slow = slow.next;
        if (fast === slow) {
            let ptr: ListNode | null | undefined = head;
            while (ptr !== slow) {
                // if (ptr === null || slow === null)
                //     return null;
                ptr = ptr?.next;
                slow = slow?.next;
            }
            return ptr as ListNode | null;
        }
    }
    return null;
};

export default detectCycle;
