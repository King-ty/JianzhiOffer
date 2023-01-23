#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

class Solution {
public:
  bool isNumber(string s) {
    s.erase(0, s.find_first_not_of(" "));
    s.erase(s.find_last_not_of(" ") + 1);
    if (s.empty() || s == "." || s == "+." || s == "-." ||
        s[s.size() - 1] == 'e' || s[s.size() - 1] == 'E' ||
        s[s.size() - 1] == '+' || s[s.size() - 1] == '-')
      return false;
    istringstream istream = istringstream(s);
    double tmp;
    istream >> tmp;
    return istream.eof();
  }
};