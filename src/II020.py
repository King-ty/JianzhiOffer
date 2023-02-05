# 枚举中心法
class Solution:
    def countSubstrings(self, s: str) -> int:
        n = len(s)
        ret = 0
        for i in range(n*2-1):
            (ll, rr) = (i//2, (i+1)//2)
            while ll >= 0 and rr < n and s[ll] == s[rr]:
                ret += 1
                ll -= 1
                rr += 1
        return ret
