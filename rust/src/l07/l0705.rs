#[derive(Debug)]
pub struct MyHashSet {
    buckets: Vec<Bucket>,
    count: usize,
}

#[derive(Debug, Clone)]
struct Bucket {
    data: [Option<i32>; 8],
}

/*
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet {
            buckets: vec![Bucket { data: [None; 8] }; 4],
            count: 0,
        }
    }

    pub fn add(&mut self, key: i32) {
        let hash = (key as usize) % self.buckets.len();
        for v in self.buckets[hash].data.iter_mut().flatten() {
            if *v == key {
                return;
            }
        }

        for v in self.buckets[hash].data.iter_mut() {
            if v.is_none() {
                *v = Some(key);
                self.count += 1;
                return;
            }
        }

        for _ in 0..10 {
            let mut new_buckets = vec![Bucket { data: [None; 8] }; self.buckets.len() * 2];
            for (i, bucket) in self.buckets.iter().enumerate() {
                let datas = [i, i + self.buckets.len()];
                let mut pointer = [0, 0];
                for inner in bucket.data.iter().flatten() {
                    let hash = (*inner as usize) % new_buckets.len();
                    let target = hash / self.buckets.len();
                    let bucket = &mut new_buckets[datas[target]];
                    bucket.data[pointer[target]] = Some(*inner);
                    pointer[target] += 1;
                }
            }
            self.buckets = new_buckets;

            let new_hash = (key as usize) % self.buckets.len();
            for v in self.buckets[new_hash].data.iter_mut() {
                if v.is_none() {
                    *v = Some(key);
                    self.count += 1;
                    return;
                }
            }
        }
        unreachable!("add failed")
    }

    pub fn remove(&mut self, key: i32) {
        let hash = (key as usize) % self.buckets.len();
        for v in self.buckets[hash].data.iter_mut() {
            if let Some(inner) = v {
                if *inner == key {
                    *v = None;
                    self.count -= 1;
                    return;
                }
            }
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let hash = (key as usize) % self.buckets.len();
        for v in self.buckets[hash].data.iter().flatten() {
            if *v == key {
                return true;
            }
        }
        false
    }
}

impl Default for MyHashSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actions = [
            "MyHashSet",
            "add",
            "add",
            "contains",
            "contains",
            "add",
            "contains",
            "remove",
            "contains",
        ];
        let values = [0, 1, 2, 1, 3, 2, 2, 2, 2];
        let except = [false, false, false, true, false, false, true, false, false];

        let mut obj = MyHashSet::new();
        for i in 1..actions.len() {
            dbg!((i, actions[i], values[i], except[i]));
            match actions[i] {
                "add" => obj.add(values[i]),
                "remove" => obj.remove(values[i]),
                "contains" => assert_eq!(obj.contains(values[i]), except[i]),
                _ => unreachable!(),
            }
            dbg!(&obj);
        }
    }
}
