#[derive(Debug)]
pub struct MyHashMap {
    buckets: Vec<Bucket>,
    count: usize,
}

#[derive(Debug, Clone)]
struct Bucket {
    data: [Option<(i32, i32)>; 8],
}

/*
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap {
            buckets: vec![Bucket { data: [None; 8] }; 4],
            count: 0,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let hash = (key as usize) % self.buckets.len();
        for (v_key, v_value) in self.buckets[hash].data.iter_mut().flatten() {
            if *v_key == key {
                *v_value = value;
                return;
            }
        }

        for v in self.buckets[hash].data.iter_mut() {
            if v.is_none() {
                *v = Some((key, value));
                self.count += 1;
                return;
            }
        }

        for _ in 0..10 {
            let mut new_buckets = vec![Bucket { data: [None; 8] }; self.buckets.len() * 2];
            for (i, bucket) in self.buckets.iter().enumerate() {
                let datas = [i, i + self.buckets.len()];
                let mut pointer = [0, 0];
                for entry in bucket.data.iter().flatten() {
                    let hash = (entry.0 as usize) % new_buckets.len();
                    let target = hash / self.buckets.len();
                    let bucket = &mut new_buckets[datas[target]];
                    bucket.data[pointer[target]] = Some(*entry);
                    pointer[target] += 1;
                }
            }
            self.buckets = new_buckets;

            let new_hash = (key as usize) % self.buckets.len();
            for v in self.buckets[new_hash].data.iter_mut() {
                if v.is_none() {
                    *v = Some((key, value));
                    self.count += 1;
                    return;
                }
            }
        }

        unreachable!("add failed");
    }

    pub fn get(&self, key: i32) -> i32 {
        let hash = (key as usize) % self.buckets.len();
        for entry in self.buckets[hash].data.iter().flatten() {
            if entry.0 == key {
                return entry.1;
            }
        }
        -1
    }

    pub fn remove(&mut self, key: i32) {
        let hash = (key as usize) % self.buckets.len();
        for v in self.buckets[hash].data.iter_mut() {
            if let Some(entry) = v {
                if entry.0 == key {
                    *v = None;
                    self.count -= 1;
                    return;
                }
            }
        }
    }
}

impl Default for MyHashMap {
    fn default() -> Self {
        Self::new()
    }
}