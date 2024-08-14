use std::collections::HashMap;

struct MagicDictionary {
    words: HashMap<usize, Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            words: HashMap::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            let len = word.len();
            let entry = self.words.entry(len).or_insert(Vec::new());
            entry.push(word);
        }
    }

    fn search(&self, search_word: String) -> bool {
        let len = search_word.len();

        let Some(words) = self.words.get(&len) else {
            return false;
        };

        for word in words {
            let miss_matchs = search_word.chars().zip(word.chars()).filter(|(a, b)| a != b).count();
            if miss_matchs == 1 {
                return true;
            }
        }

        false
    }
}
