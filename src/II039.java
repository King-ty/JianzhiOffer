import java.util.Stack;

public class II039 {

}

class Solution {
    public int largestRectangleArea(int[] heights) {
        int ret = 0, n = heights.length;
        Stack<Integer> st = new Stack<>();
        int[] left = new int[n];
        int[] right = new int[n];
        for (int i = 0; i < n; ++i) {
            int height = heights[i];
            while (!st.empty() && heights[st.peek()] >= height) {
                st.pop();
            }
            left[i] = st.empty() ? -1 : st.peek();
            st.push(i);
        }
        st.clear();
        for (int i = n - 1; i >= 0; --i) {
            int height = heights[i];
            while (!st.empty() && heights[st.peek()] >= height) {
                st.pop();
            }
            right[i] = st.empty() ? n : st.peek();
            st.push(i);
        }
        for (int i = 0; i < n; ++i) {
            ret = Math.max(ret, (right[i] - left[i] - 1) * heights[i]);
        }
        return ret;
    }
}
