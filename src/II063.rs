struct Solution;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut p = self;
        for i in word.bytes().map(|ch| (ch - b'a') as usize) {
            // p = p.children[i].get_or_insert(Box::new(Self::new()));
            p = p.children[i].get_or_insert_with(|| Box::new(Trie::new())); // 这比上面快很多，为什么？
        }
        p.is_end = true;
    }

    fn replace(&self, word: &mut String) {
        let mut p = self;
        let mut cnt = 0;
        for i in word.bytes().map(|ch| (ch - b'a') as usize) {
            match &p.children[i] {
                Some(v) => {
                    cnt += 1;
                    if v.is_end {
                        break;
                    }
                    p = v;
                }
                _ => return,
            }
        }
        word.truncate(cnt);
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in dictionary {
            trie.insert(word);
        }
        let mut words: Vec<String> = sentence.split(' ').map(|x| x.to_string()).collect();
        for word in &mut words {
            trie.replace(word);
        }
        words.join(" ")
    }
}
