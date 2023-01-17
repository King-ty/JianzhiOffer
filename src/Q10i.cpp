class Solution {
public:
  static const int MOD = 1e9 + 7;
  int fib(int n) {
    int a = 0, b = 1;
    for (int i = 1; i <= n; ++i) {
      int temp = b;
      b = (a + b) % MOD;
      a = temp;
    }
    return a;
  }
};