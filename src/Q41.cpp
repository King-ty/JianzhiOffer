// 对顶堆
#include <iostream>
#include <queue>
#include <vector>

using namespace std;

class MedianFinder {
private:
  priority_queue<int> queMin;
  // priority_queue<int, vector<int>, less<int>> queMin;
  priority_queue<int, vector<int>, greater<int>> queMax;

public:
  /** initialize your data structure here. */
  MedianFinder() {}

  void addNum(int num) {
    if (queMin.empty() || queMin.top() > num) {
      queMin.push(num);
      if (queMin.size() > queMax.size() + 1) {
        queMax.push(queMin.top());
        queMin.pop();
      }
    } else {
      queMax.push(num);
      if (queMax.size() > queMin.size()) {
        queMin.push(queMax.top());
        queMax.pop();
      }
    }
  }

  double findMedian() {
    if (queMin.size() > queMax.size()) {
      return double(queMin.top());
    } else {
      return double(queMin.top() + queMax.top()) / 2;
    }
  }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */