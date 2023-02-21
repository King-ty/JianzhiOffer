#include <array>
#include <cstddef>
#include <iostream>
#include <vector>
using namespace std;

class Trie {
  array<Trie*, 26> children;
  bool is_end;

  Trie* searchPrefix(const string& prefix) {
    Trie* p = this;
    for (char ch : prefix) {
      ch -= 'a';
      if (p->children[ch] == nullptr) return nullptr;
      p = p->children[ch];
    }
    return p;
  }

public:
  /** Initialize your data structure here. */
  Trie(bool is_end = false) : children(), is_end(is_end) {} // array必须显式初始化

  /** Inserts a word into the trie. */
  void insert(string word) {
    Trie* p = this;
    for (char ch : word) {
      ch -= 'a';
      if (p->children[ch] == nullptr) {
        p->children[ch] = new Trie();
      }
      p = p->children[ch];
    }
    p->is_end = true;
  }

  /** Returns if the word is in the trie. */
  bool search(string word) {
    auto res = searchPrefix(word);
    return res != nullptr && res->is_end;
  }

  /** Returns if there is any word in the trie that starts with the given prefix. */
  bool startsWith(string prefix) { return searchPrefix(prefix) != nullptr; }
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */

int main() {
  Trie* obj = new Trie();
  obj->insert("apple");
  bool param_2 = obj->search("app");
  bool param_3 = obj->startsWith("app");
  cout << param_2 << ' ' << param_3 << endl;
}
