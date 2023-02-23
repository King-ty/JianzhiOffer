struct Solution;

use std::cmp::max;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 2],
    flags: [bool; 2],
    val: i32,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, num: i32) {
        let mut p = self;
        let mut index = 1 << 30;
        while index > 0 {
            let i = ((index & num) / index) as usize;
            p = p.children[i].get_or_insert_with(|| Box::new(Trie::new()));
            p.flags[0] = true;
            p.val = i as i32;
            index >>= 1;
        }
        // p.flags[0] = true;
        index = 1 << 30;
        while index > 0 {
            let i = if (index & num) == 0 { 1 } else { 0 };
            p = p.children[i].get_or_insert_with(|| Box::new(Trie::new()));
            p.flags[1] = true;
            p.val = i as i32;
            index >>= 1;
        }
        // p.flags[1] = true;
    }

    fn getRes(&self) -> i32 {
        fn search(p: &Option<Box<Trie>>, res: i32) -> i32 {
            match p {
                Some(trie) => {
                    if trie.flags[0] && trie.flags[1] {
                        max(
                            search(&trie.children[0], (res << 1) + 1),
                            search(&trie.children[1], (res << 1) + 1),
                        )
                    } else if trie.flags[0] {
                        max(
                            search(&trie.children[0], (res << 1) + trie.val),
                            search(&trie.children[1], (res << 1) + trie.val),
                        )
                    } else {
                        res
                    }
                }
                None => res,
            }
        }
        max(search(&self.children[0], 0), search(&self.children[1], 0))
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for num in nums {
            trie.insert(num);
        }
        trie.getRes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II067() {
        dbg!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    }
}
