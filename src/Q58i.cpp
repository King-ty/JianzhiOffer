#include <iostream>
#include <sstream>
#include <stack>
#include <type_traits>
#include <vector>


using namespace std;

class Solution {
public:
  string reverseWords(string s) {
    istringstream istream = istringstream(s);
    string word;
    stack<string> st;
    while (istream >> word) {
      st.push(word);
    }
    ostringstream ostream;
    bool flag = false;
    while (!st.empty()) {
      if (flag)
        ostream << ' ';
      flag = true;
      ostream << st.top();
      st.pop();
    }
    return ostream.str();
  }
};