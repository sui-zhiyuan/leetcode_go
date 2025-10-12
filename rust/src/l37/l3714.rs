use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn longest_balanced(s: String) -> i32 {
    let s = s
        .chars()
        .map(|c| {
            assert!(('a'..='c').contains(&c));
            (c as u32 - 'a' as u32) as u8
        })
        .collect::<Vec<_>>();

    let mut result = 0;

    result = result.max(longest_single(&s));
    for i in [(0, 1), (1, 2), (0, 2)] {
        result = result.max(longest_double(&s, i.0, i.1));
    }
    result = result.max(longest_triple(&s));

    result
}

fn longest_single(s: &[u8]) -> i32 {
    let mut result = 0;
    let mut start = 0;
    while start < s.len() {
        let mut end = start + 1;
        while end < s.len() && s[end] == s[start] {
            end += 1;
        }

        result = result.max(end - start);
        start = end;
    }

    result as i32
}

fn longest_double(s: &[u8], c1: u8, c2: u8) -> i32 {
    let c_rest = 3 - c1 - c2;

    let mut first_find = HashMap::<(i32, i32), usize>::new();
    first_find.insert((0, 0), 0);
    let mut result = 0;

    let mut count_diff = 0i32;
    let mut count_rest = 0i32;
    for (i, &c) in s.iter().enumerate() {
        match c {
            c if c == c1 => count_diff += 1,
            c if c == c2 => count_diff -= 1,
            c if c == c_rest => count_rest += 1,
            _ => unreachable!(),
        }

        let i = i + 1; // count_diff and count_rest for 0..i

        match first_find.entry((count_diff, count_rest)) {
            Entry::Vacant(v) => _ = *v.insert(i),
            Entry::Occupied(o) => result = result.max(i - o.get()),
        }
    }

    result as i32
}

fn longest_triple(s: &[u8]) -> i32 {
    let mut first_find = HashMap::<(i32, i32), usize>::new();
    first_find.insert((0, 0), 0);
    let mut result = 0;

    let mut diff01 = 0i32;
    let mut diff12 = 0i32;
    for (i, &c) in s.iter().enumerate() {
        match c {
            0 => diff01 += 1,
            1 => {
                diff01 -= 1;
                diff12 += 1;
            }
            2 => diff12 -= 1,
            _ => unreachable!(),
        }
        let i = i + 1; // count_diff and count_rest for 0..i

        match first_find.entry((diff01, diff12)) {
            Entry::Vacant(v) => _ = *v.insert(i),
            Entry::Occupied(o) => result = result.max(i - o.get()),
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        input: &'static str,
        expect: i32,
    }

    fn test(c: Case) {
        assert_eq!(c.expect, longest_balanced(c.input.to_owned()))
    }

    parameter_tests! {
        test,
        (case1: Case {
            input: "abbac",
            expect: 4,
        }),
        (case2: Case {
            input: "aabcc",
            expect: 3,
        }),
        (case3: Case {
            input: "aba",
            expect: 2,
        }),
    }
}
