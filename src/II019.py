class Solution:
    def _validPalindrome(self, s: str, i: int, j: int) -> bool:
        while i < j:
            if s[i] != s[j]:
                return False
            i += 1
            j -= 1
        return True

    def validPalindrome(self, s: str) -> bool:
        (i, j) = (0, len(s)-1)
        while i < j:
            if s[i] != s[j]:
                return self._validPalindrome(s, i+1, j) or self._validPalindrome(s, i, j-1)
            i += 1
            j -= 1
        return True
