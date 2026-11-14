#[allow(dead_code)]
pub struct Trie {
    val: char,
    children: Vec<Trie>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::str::Chars;

impl Trie {
    pub fn new() -> Self {
        Trie {
            val: '*',
            children: vec![],
        }
    }

    pub fn insert(&mut self, word: String) {
        self.insert_helper(&mut word.chars());
    }

    fn insert_helper(&mut self, chars: &mut Chars) {
        if let Some(c) = chars.next() {
            let mut inserted = false;
            for child in &mut self.children {
                if child.val == c {
                    inserted = true;
                    child.insert_helper(chars);
                }
            }
            if !inserted {
                let mut pointer = self;
                let mut n = pointer.children.len();
                pointer.children.push(Trie {
                    val: c,
                    children: vec![],
                });
                pointer = &mut pointer.children[n];
                n = pointer.children.len();
                while let Some(c) = chars.next() {
                    pointer.children.push(Trie {
                        val: c,
                        children: vec![],
                    });
                    pointer = &mut pointer.children[n];
                    n = pointer.children.len();
                }
                pointer.children.push(Trie {
                    val: '*',
                    children: vec![],
                });
            }
        } else {
            self.children.push(Trie {
                val: '*',
                children: vec![],
            });
        }
    }

    pub fn search(&self, word: String) -> bool {
        self.search_helper(&mut word.chars())
    }

    fn search_helper(&self, chars: &mut Chars) -> bool {
        if let Some(c) = chars.next() {
            for child in &self.children {
                if child.val == c {
                    return child.search_helper(chars);
                }
            }
            false
        } else {
            for c in &self.children {
                if c.val == '*' {
                    return true;
                }
            }
            false
        }
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.prefix_helper(&mut prefix.chars())
    }

    fn prefix_helper(&self, chars: &mut Chars) -> bool {
        return if let Some(c) = chars.next() {
            for child in &self.children {
                if child.val == c {
                    return child.prefix_helper(chars);
                }
            }
            false
        } else {
            true
        };
    }
}