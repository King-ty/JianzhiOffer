class Solution:
    def minWindow(self, s: str, t: str) -> str:
        (m, n) = (len(s), len(t))
        if m < n:
            return ""
        cnt = [0]*60
        diff = 0
        min_len = m+1
        ret = ""
        for c in t:
            c = ord(c)-ord('A')
            if cnt[c] == 0:
                diff += 1
            cnt[c] -= 1
        ll = 0
        for r in range(m):
            x = ord(s[r])-ord('A')
            cnt[x] += 1
            if cnt[x] == 0:
                diff -= 1
            while ll <= r and cnt[ord(s[ll])-ord('A')] > 0:
                cnt[ord(s[ll])-ord('A')] -= 1
                ll += 1
            if diff == 0 and min_len > r-ll+1:
                min_len = r-ll+1
                ret = s[ll:r+1]

        return ret


solu = Solution()
print(solu.minWindow("A", "B"))
