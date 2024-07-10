use std::mem;

pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    let mut root = Node {
        next: Vec::<char>::new(),
        // cost: None,
        max_len: 0,
        children: Vec::<Node>::new(),
    };

    let mut word_cost = words
        .into_iter()
        .zip(costs)
        .collect::<Vec<(String, i32)>>();
    word_cost.sort();

    let mut prev = "".to_string();
    for (word, _cost) in word_cost {
        if word == prev {
            continue;
        }
        prev.clone_from(&word);
        let w_len = word.len();
        let mut array_word = word.chars().collect::<Vec<char>>();
        let mut word: &mut [char] = &mut array_word;
        let curr = &mut root;
        while !word.is_empty() {
            let mut prefix_len = 0;
            let mut index = 0;
            for (i, node) in curr.children.iter().enumerate() {
                for (j, c) in word.iter().enumerate() {
                    if (j >= node.next.len() || node.next[j] != *c) && j >= 1 {
                        prefix_len = j;
                        index = i;
                    }
                }
            }

            if prefix_len > 0 {
                let mut new_node = Node {
                    next: word[..prefix_len].to_vec(),
                    // cost: None,
                    max_len: curr.children[index].max_len.max(w_len),
                    children: Vec::<Node>::new(),
                };
                mem::swap(&mut curr.children[index], &mut new_node);
                curr.children[index].children.push(new_node);
                continue;
            }

            curr.children.push(Node {
                next: word.to_owned(),
                // cost: Some(cost),
                max_len: w_len,
                children: Vec::<Node>::new(),
            });
            word = &mut word[0..0];
        }
    }

    root.sort();

    let _target = target.chars().collect::<Vec<char>>();
    let _min_cost = i32::MAX;
    todo!()
    // find_match(&root, &root, &target, 0, &mut min_cost);
    // if min_cost == i32::MAX {
    //     -1
    // } else {
    //     min_cost
    // }
}

struct Node {
    next: Vec<char>,
    // cost: Option<i32>,
    max_len: usize,
    children: Vec<Node>,
}

// fn find_match(root: &Node, curr: &Node, target: &[char], cost: i32, min_cost: &mut i32) {
//     if cost > *min_cost {
//         return;
//     }

//     if target.is_empty() && curr.cost.is_some() {
//         *min_cost = (*min_cost).min(cost + curr.cost.unwrap());
//         return;
//     }

//     if target.is_empty() {
//         return;
//     }

//     for node in curr.children.iter() {
//         if node.next == target[0] {
//             find_match(root, node, &target[1..], cost, min_cost);
//         }
//     }

//     if let Some(c) = curr.cost {
//         find_match(root, root, target, cost + c, min_cost);
//     }
// }

impl Node {
    fn sort(&mut self) {
        self.children
            .sort_by(|a, b| a.max_len.cmp(&b.max_len).reverse());
        for node in self.children.iter_mut() {
            node.sort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let target = "abcdef".to_string();
        let words = vec!["abdef", "abc", "d", "def", "ef"];
        let words = words
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let costs = vec![100, 1, 1, 10, 5];
        let res = minimum_cost(target, words, costs);
        assert_eq!(res, 7);
    }
}
