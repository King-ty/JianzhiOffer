#include <array>
#include <cstddef>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class MapSum {
  array<MapSum*, 26> children;
  int val_;
  unordered_map<string, int> cnt;

public:
  /** Initialize your data structure here. */
  MapSum() : children(), val_(0) {}

  void insert(string key, int val) {
    int delta = val;
    if (cnt.count(key)) {
      delta -= cnt[key];
    }
    cnt[key] = val;
    auto p = this;
    for (char ch : key) {
      ch -= 'a';
      if (p->children[ch] == nullptr) {
        p->children[ch] = new MapSum();
      }
      p = p->children[ch];
      p->val_ += delta;
    }
  }

  int sum(string prefix) {
    auto p = this;
    for (char ch : prefix) {
      ch -= 'a';
      if (p->children[ch] == nullptr) return 0;
      p = p->children[ch];
    }
    return p->val_;
  }
};

// class MapSum {
//   array<MapSum*, 26> children;
//   int val_;

//   MapSum* findPrefix(const string& prefix) {
//     auto p = this;
//     for (char ch : prefix) {
//       ch -= 'a';
//       if (p->children[ch] == nullptr) return nullptr;
//       p = p->children[ch];
//     }
//     return p;
//   }

//   int treeSum() {
//     int res = val_;
//     for (auto child : children) {
//       if (child) {
//         res += child->treeSum();
//       }
//     }
//     return res;
//   }

// public:
//   /** Initialize your data structure here. */
//   MapSum() : children(), val_(0) {}

//   void insert(string key, int val) {
//     auto p = this;
//     for (char ch : key) {
//       ch -= 'a';
//       if (p->children[ch] == nullptr) {
//         p->children[ch] = new MapSum();
//       }
//       p = p->children[ch];
//     }
//     p->val_ = val;
//   }

//   int sum(string prefix) {
//     auto p = this->findPrefix(prefix);
//     if (p) return p->treeSum();
//     return 0;
//   }
// };

/**
 * Your MapSum object will be instantiated and called as such:
 * MapSum* obj = new MapSum();
 * obj->insert(key,val);
 * int param_2 = obj->sum(prefix);
 */
