// 这题其实想到了这种做法，一开始想偷懒不自己写一个双链表，没找到方法。用Rust实现过于困难，于是摆烂了
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

struct DListNode {
  int key, value;
  DListNode *prev, *next;
  DListNode(int key = 0, int value = 0, DListNode *prev = nullptr,
            DListNode *next = nullptr)
      : key(key), value(value), prev(prev), next(next) {}
};

class LRUCache {
  DListNode *head, *tail;
  unordered_map<int, DListNode *> mp;
  int capacity, size;

  void addToHead(DListNode *node) {
    node->next = head->next;
    node->prev = head;
    head->next->prev = node;
    head->next = node;
  }
  void moveToHead(DListNode *node) {
    node->prev->next = node->next;
    node->next->prev = node->prev;
    addToHead(node);
  }
  void removeTail() {
    auto node = tail->prev;
    tail->prev = node->prev;
    node->prev->next = tail;
    mp.erase(node->key);
    delete node;
  }

public:
  LRUCache(int capacity)
      : head(new DListNode()), tail(new DListNode()), capacity(capacity),
        size(0) {
    head->next = tail;
    tail->prev = head;
  }

  int get(int key) {
    if (!mp.count(key))
      return -1;
    auto node = mp.at(key);
    moveToHead(node);
    return node->value;
  }

  void put(int key, int value) {
    if (!mp.count(key)) {
      auto node = new DListNode(key, value);
      addToHead(node);
      mp.insert({key, node});
      if (++size > capacity) {
        removeTail();
        --size;
      }
    } else {
      auto node = mp.at(key);
      node->value = value;
      moveToHead(node);
    }
  }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */

int main() {
  LRUCache cache(2);
  cache.put(1, 1);
  cache.put(2, 2);
  cout << cache.get(1) << endl;
  cache.put(3, 3);
  cout << cache.get(2) << endl;
  cache.put(4, 4);
  cout << cache.get(1) << endl;
  cout << cache.get(3) << endl;
  cout << cache.get(4) << endl;

  return 0;
}
