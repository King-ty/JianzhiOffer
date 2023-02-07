public class II029 {

}

// Definition for a Node.
class Node {
    public int val;
    public Node next;

    public Node() {
    }

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, Node _next) {
        val = _val;
        next = _next;
    }
};

class Solution {
    public Node insert(Node head, int insertVal) {
        if (head == null) {
            var ret = new Node(insertVal);
            ret.next = ret;
            return ret;
        }
        var p = head.next;
        while (p != head) {
            if ((p.val <= insertVal && p.next.val >= insertVal)
                    || (p.next.val < p.val && (insertVal >= p.val || insertVal <= p.next.val))) {
                break;
            }
            p = p.next;
        }
        p.next = new Node(insertVal, p.next);
        return head;
    }
}
