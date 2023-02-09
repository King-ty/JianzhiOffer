public class II034 {

}

class Solution {
    int[] ord;

    boolean isSmallEq(String w1, String w2) {
        int i = 0, m = w1.length(), n = w2.length();
        for (; i < m && i < n; ++i) {
            int x = w1.charAt(i) - 'a', y = w2.charAt(i) - 'a';
            if (ord[x] != ord[y])
                return ord[x] < ord[y];
        }
        return i < n || m == n;
    }

    public boolean isAlienSorted(String[] words, String order) {
        ord = new int[26];
        for (int i = 0; i < order.length(); ++i) {
            ord[order.charAt(i) - 'a'] = i;
        }
        for (int i = 1; i < words.length; ++i) {
            if (!isSmallEq(words[i - 1], words[i])) {
                return false;
            }
        }
        return true;
    }
}
