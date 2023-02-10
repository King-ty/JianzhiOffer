from typing import List


class Solution:
    def maximalRectangle(self, matrix: List[str]) -> int:
        m = len(matrix)
        if m == 0:
            return 0
        n = len(matrix[0])
        if n == 0:
            return 0
        ret = 0
        # cnt = [[0]*n]*m # 注意这样构建出来的二维数组，各行是引用关系，都一样
        cnt = [[0]*n for _ in range(m)]  # 需要这样构建二维数组！
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == '1':
                    cnt[i][j] = 1 if j == 0 else cnt[i][j-1]+1
        for j in range(n):
            st = []
            left = [0]*m
            for i in range(m):
                while st and cnt[st[len(st)-1]][j] >= cnt[i][j]:
                    st.pop()
                left[i] = -1 if not st else st[len(st)-1]
                st.append(i)
            st = []
            for i in reversed(range(m)):
                while st and cnt[st[len(st)-1]][j] >= cnt[i][j]:
                    st.pop()
                right = m if not st else st[len(st)-1]
                st.append(i)
                ret = max(ret, (right-left[i]-1)*cnt[i][j])
        return ret


matrix = ["10100", "10111", "11111", "10010"]
print(Solution().maximalRectangle(matrix))
