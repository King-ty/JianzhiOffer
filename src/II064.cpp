#include <array>
#include <iostream>
#include <vector>

using namespace std;

class Trie {
  array<Trie*, 26> children;
  bool is_end;

public:
  /** Initialize your data structure here. */
  Trie(bool is_end = false) : children(), is_end(is_end) {}

  /** Inserts a word into the trie. */
  void insert(const string& word) {
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
  bool searchWithOneFault(const string& word, int index = 0, bool flag = false) {
    if (index == int(word.size())) {
      return flag && this->is_end;
    }
    char ch = word[index] - 'a';
    if (this->children[ch] != nullptr
        && this->children[ch]->searchWithOneFault(word, index + 1, flag)) {
      return true;
    }
    if (!flag) {
      for (int i = 0; i < 26; ++i) {
        if (i == ch) continue;
        if (this->children[i] != nullptr
            && this->children[i]->searchWithOneFault(word, index + 1, true)) {
          return true;
        }
      }
    }
    return false;
  };
};

class MagicDictionary {
  Trie* trie;

public:
  /** Initialize your data structure here. */
  MagicDictionary() : trie(new Trie()) {}

  void buildDict(vector<string> dictionary) {
    for (auto word : dictionary) {
      trie->insert(word);
    }
  }

  bool search(string searchWord) { return trie->searchWithOneFault(searchWord); }
};

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * MagicDictionary* obj = new MagicDictionary();
 * obj->buildDict(dictionary);
 * bool param_2 = obj->search(searchWord);
 */
