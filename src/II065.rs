use std::string;

struct Solution;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut p = self;
        for i in word.bytes().rev().map(|x| (x - b'a') as usize) {
            p = p.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    fn leaves_num(&self, dep: i32) -> i32 {
        let mut ret = 0;
        for child in self.children.as_ref() {
            if let Some(trie) = child {
                ret += trie.leaves_num(dep + 1);
            }
        }
        if ret == 0 {
            dep
        } else {
            ret
        }
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        trie.leaves_num(1)
    }
}
