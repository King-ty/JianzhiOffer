#include <functional>
#include <iostream>
#include <set>
#include <type_traits>
#include <vector>

using namespace std;

// 使用set偷懒法，其实线段树更快（大概）
class MyCalendar {
  set<pair<int, int>> st;

public:
  MyCalendar() {}

  bool book(int start, int end) {
    auto index = st.lower_bound({start, -1});
    if (index != st.end() && index->first == start) ++index;
    if (index != st.end() && index->second < end) return false;
    st.insert({end, start});
    return true;
  }
};

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */

int main() {
  MyCalendar ca;
  cout << ca.book(10, 20) << endl;
  cout << ca.book(15, 25) << endl;
  cout << ca.book(20, 30) << endl;

  return 0;
}
