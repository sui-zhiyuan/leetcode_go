use std::collections::HashSet;

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut storage = vec![
        Storage {
            trusted: HashSet::new(),
            trusting: false
        };
        n as usize + 1
    ];
    for t in trust.iter() {
        let [truster, trustee] = t[..] else {
            panic!("Invalid input");
        };

        storage[trustee as usize].trusted.insert(truster);
        storage[truster as usize].trusting = true;
    }

    storage
        .iter()
        .enumerate()
        .skip(1)
        .find_map(|(i, s)| {
            if !s.trusting && s.trusted.len() == n as usize - 1 {
                Some(i as i32)
            } else {
                None
            }
        })
        .unwrap_or(-1)
}

#[derive(Debug, Clone)]
struct Storage {
    trusted: HashSet<i32>,
    trusting: bool,
}
