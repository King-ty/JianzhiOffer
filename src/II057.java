import java.util.HashMap;
import java.util.Map;

public class II057 {

}

class Solution {
    int getID(int x, long w) {
        return (int) (x < 0 ? (x + 1L) / w - 1 : x / w);
    }

    public boolean containsNearbyAlmostDuplicate(int[] nums, int k, int t) {
        Map<Integer, Long> mp = new HashMap<>();
        long w = t + 1L;
        for (int i = 0; i < nums.length; ++i) {
            var id = getID(nums[i], w);
            if (mp.containsKey(id))
                return true;
            if (mp.containsKey(id - 1) && Math.abs(nums[i] - mp.get(id - 1)) < w)
                return true;
            if (mp.containsKey(id + 1) && Math.abs(nums[i] - mp.get(id + 1)) < w)
                return true;
            mp.put(id, (long) nums[i]);
            if (i >= k) {
                mp.remove(getID(nums[i - k], w));
            }
        }
        return false;
    }
}
