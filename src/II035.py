from typing import List


class Solution:
    def findMinDifference(self, timePoints: List[str]) -> int:
        if len(timePoints) > 24*60:
            # 鸽巢原理
            return 0
        times: List[int] = []
        for tp in timePoints:
            tm = tp.split(':')
            times.append(int(tm[0])*60+int(tm[1]))
        times.sort()
        ret = times[1]-times[0]
        for i in range(len(times)-1):
            ret = min(ret, times[i+1]-times[i])
        return min(ret, times[0]+24*60-times[len(times)-1])
