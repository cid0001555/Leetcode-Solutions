use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let n = s.len();

        let dict: HashSet<String> = word_dict.into_iter().collect();

        let mut memo: HashMap<usize, Vec<String>> = HashMap::new();

        fn dfs(
            start: usize,
            s: &str,
            dict: &HashSet<String>,
            memo: &mut HashMap<usize, Vec<String>>,
        ) -> Vec<String> {
            if start == s.len() {
                return vec![String::new()];
            }

            if let Some(result) = memo.get(&start) {
                return result.clone();
            }

            let mut result = Vec::new();

            for end in start + 1..=s.len() {
                let word = &s[start..end];

                if dict.contains(word) {
                    let suffixes = dfs(end, s, dict, memo);

                    for suffix in suffixes {
                        if suffix.is_empty() {
                            result.push(word.to_string());
                        } else {
                            result.push(format!("{} {}", word, suffix));
                        }
                    }
                }
            }

            memo.insert(start, result.clone());
            result
        }

        dfs(0, &s, &dict, &mut memo)
    }
}
