public class II026 {

}

// Definition for singly-linked list.
class ListNode {
    int val;
    ListNode next;

    ListNode() {
    }

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }
}

class Solution {
    ListNode middleNode(ListNode head) {
        ListNode fast = head, slow = head;
        while (fast.next != null && fast.next.next != null) {
            fast = fast.next.next;
            slow = slow.next;
        }
        return slow;
    }

    ListNode reverseList(ListNode head) {
        ListNode ret = null;
        var p = head;
        while (p != null) {
            var temp = p.next;
            p.next = ret;
            ret = p;
            p = temp;
        }
        return ret;
    }

    void mergeList(ListNode l1, ListNode l2) {
        var p = l1;
        while (l1 != null && l2 != null) {
            l1 = p.next;
            p.next = l2;
            p = p.next;
            l2 = p.next;
            p.next = l1;
            p = p.next;
        }
    }

    public void reorderList(ListNode head) {
        ListNode mid = middleNode(head);
        ListNode head2 = mid.next;
        mid.next = null;
        head2 = reverseList(head2);
        mergeList(head, head2);
    }
}
