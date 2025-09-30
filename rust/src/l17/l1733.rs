use std::collections::{HashMap, HashSet};
use std::iter;

pub fn minimum_teachings(_n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    let languages = iter::once(HashSet::new())
        .chain(languages.into_iter().map(HashSet::<i32>::from_iter))
        .collect::<Vec<_>>();
    let mut need_person = HashSet::new();

    for friendship in friendships {
        let &[f0, f1] = &friendship[..] else {
            panic!("invalid friendship");
        };

        let f0 = f0 as usize;
        let f1 = f1 as usize;

        if languages[f0].is_disjoint(&languages[f1]) {
            dbg!(("mismatch", f0, f1));
            need_person.insert(f0);
            need_person.insert(f1);
        }
    }

    let mut known_languages = HashMap::<i32, i32>::new();

    for &p in need_person.iter() {
        for &l in languages[p].iter() {
            let count = known_languages.entry(l).or_default();
            *count += 1;
        }
    }

    need_person.len() as i32 - known_languages.values().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_grid;
    use crate::parameter_tests;

    struct Case {
        n: i32,
        languages: &'static str,
        friendships: &'static str,
        expected: i32,
    }

    fn test(c: Case) {
        let language = parse_grid::<i32>(c.languages).unwrap();
        let friendship = parse_grid::<i32>(c.friendships).unwrap();
        assert_eq!(c.expected, minimum_teachings(c.n, language, friendship))
    }

    parameter_tests! {
        test,
        (case1: Case {
            n: 10,
            languages: "[[1],[2],[1,2]]",
            friendships: "[[1,2],[1,3],[2,3]]",
            expected: 1,
        }),
        (case2: Case {
            n: 3,
            languages: "[[2],[1,3],[1,2],[3]]",
            friendships: "[[1,4],[1,2],[3,4],[2,3]]",
            expected: 2,
        }),
        (case3: Case {
            n: 2,
            languages: "[[2],[1],[2,1],[1],[1,2],[1],[2],[1],[1],[2],[1,2],[1,2],[1,2],[2,1],[1],[2],[1,2]]",
            friendships: "[[15,16],[4,13],[3,16],[5,14],[1,7],[2,11],[3,15],[4,16],[7,9],[6,13],[6,16],[4,10],[6,9],[5,6],[7,12],[6,12],[3,7],[4,7],[8,10]]",
            expected: 3,
        }),
    }
}
