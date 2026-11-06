use std::collections::{HashMap,VecDeque,HashSet};
impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        if !word_list.contains(&end_word) { return vec![]; }
        if !word_list.contains(&begin_word) { word_list.push(begin_word.clone()) };
        let mut graph:HashMap<&str,Vec<&str>> = HashMap::new();
        for (idx,word) in word_list.iter().enumerate() {
            word_list.iter().skip(idx)
                .map(|w| w.as_str())
                .filter(|w| word.chars().zip(w.chars()).filter(|(c1, c2)| c1 != c2).count()==1)
                .for_each(|w| {
                    graph.entry(word).or_insert(vec![]).push(w);
                    graph.entry(w).or_insert(vec![]).push(word);
                });
        }
        let mut queue = VecDeque::from_iter(vec![(1,begin_word.as_str())]);
        let mut visited:HashSet<&str> = HashSet::from_iter(vec![begin_word.as_str()]);
        let mut path_len = 0;
        while !queue.is_empty() {
            let (len,cur_word) = queue.pop_front().unwrap();
            if cur_word == end_word { path_len = len;break; }
            if !graph.contains_key(cur_word) { continue; }
            for word in &graph[cur_word] {
                if visited.contains(word) { continue };
                visited.insert(word);
                queue.push_back((len+1,word));
            }
        }
        let mut result:Vec<Vec<String>> = vec![];
        let mut stack = vec![vec![end_word.as_str()]];
        while !stack.is_empty() {
            let path = stack.pop().unwrap();
            if path.len() > path_len { continue; }
            if path[path.len()-1] == begin_word {
                result.push(path.iter().map(|w| w.to_string()).rev().collect());
                continue;
            }
            for word in &graph[&path[path.len()-1]] {
                if !visited.contains(word) || path.contains(word) || path.len() + 1 > path_len { continue };
                let mut path = path.clone();
                path.push(word);
                stack.push(path);
            }
        }
        result
    }
}
