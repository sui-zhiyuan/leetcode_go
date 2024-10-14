use std::collections::HashMap;

pub struct MagicDictionary {
    words: HashMap<usize, Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    pub fn new() -> Self {
        MagicDictionary {
            words: HashMap::new(),
        }
    }

    pub fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            let len = word.len();
            let entry = self.words.entry(len).or_default();
            entry.push(word);
        }
    }

    pub fn search(&self, search_word: String) -> bool {
        let len = search_word.len();

        let Some(words) = self.words.get(&len) else {
            return false;
        };

        for word in words {
            let miss_matches = search_word.chars().zip(word.chars()).filter(|(a, b)| a != b).count();
            if miss_matches == 1 {
                return true;
            }
        }

        false
    }
}

impl Default for MagicDictionary {
    fn default() -> Self {
        Self::new()
    }
}