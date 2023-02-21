#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut p = self;
        for i in word.bytes().map(|ch| (ch - b'a') as usize) {
            // p = p.children[i].get_or_insert(Box::new(Self::new()));
            p = p.children[i].get_or_insert_with(|| Box::new(Trie::new())); // 这比上面快很多，为什么？
        }
        p.is_end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut p = self;
        for i in word.bytes().map(|ch| (ch - b'a') as usize) {
            match &p.children[i] {
                Some(v) => p = v,
                _ => return false,
            }
        }
        p.is_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut p = self;
        for i in prefix.bytes().map(|ch| (ch - b'a') as usize) {
            match &p.children[i] {
                Some(v) => p = v,
                _ => return false,
            }
        }
        true
    }
}

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl Trie {
//     /** Initialize your data structure here. */
//     fn new() -> Self {
//         Default::default()
//     }

//     /** Inserts a word into the trie. */
//     fn insert(&mut self, word: String) {
//         let mut p = self;
//         for &ch in word.as_bytes() {
//             p = p.children[(ch - b'a') as usize].get_or_insert(Box::new(Self::new()));
//         }
//         p.is_end = true;
//     }

//     /** Returns if the word is in the trie. */
//     fn search(&self, word: String) -> bool {
//         let mut p = self;
//         for &ch in word.as_bytes() {
//             if let Some(ref node) = p.children[(ch - b'a') as usize] {
//                 p = node;
//             } else {
//                 return false;
//             }
//         }
//         p.is_end
//     }

//     /** Returns if there is any word in the trie that starts with the given prefix. */
//     fn starts_with(&self, prefix: String) -> bool {
//         let mut p = self;
//         for &ch in prefix.as_bytes() {
//             if let Some(ref node) = p.children[(ch - b'a') as usize] {
//                 p = node;
//             } else {
//                 return false;
//             }
//         }
//         true
//     }
// }
