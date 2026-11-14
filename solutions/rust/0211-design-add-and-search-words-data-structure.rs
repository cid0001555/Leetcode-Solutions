use std::collections::HashMap;

struct WordDictionary {
    children: HashMap<u8, WordDictionary>,
    is_word: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let node = word.bytes()
            .fold(self, |node, c| node.children.entry(c).or_insert(Self::new()));
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        search_rec(self, word.as_bytes())
    }
}

fn search_rec(node: &WordDictionary, word: &[u8]) -> bool {
    match word {
        [] => node.is_word,
        [b'.', tail @ ..] => node.children.values().any(|child| search_rec(child, tail)),
        [c, tail @ ..] => node.children.get(&c).is_some_and(|child| search_rec(child, tail))
    }
}
