// 有序集合双指针
#include <iostream>
#include <set>
#include <vector>

using namespace std;

class MedianFinder {
private:
  multiset<int> nums;
  multiset<int>::iterator left, right;

public:
  /** initialize your data structure here. */
  MedianFinder() { left = right = nums.end(); }

  void addNum(int num) {
    nums.insert(num);
    if (left == nums.end()) {
      left = right = nums.begin();
    } else if (left == right) {
      if (num < *left) {
        --left;
      } else {
        // num==*left时，根据multiset特点，插入右边
        ++right;
      }
    } else {
      if (num < *left) {
        --right;
        left = right; // 防止插入到了left和right中间
      } else {
        ++left;
        right = left; // 防止插入到了left和right中间
      }
    }
  }

  double findMedian() { return double(*left + *right) / 2; }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */