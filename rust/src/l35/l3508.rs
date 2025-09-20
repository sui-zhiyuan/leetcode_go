use std::collections::{HashMap, HashSet, VecDeque};

pub struct Router {
    curr_timestamp: i32,
    memory_limit: usize,
    packages: VecDeque<Package>,
    received_pack: HashSet<Package>,
    pack_by_desc: HashMap<i32, VecDeque<Package>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Package {
    source: i32,
    destination: i32,
    timestamp: i32,
}

impl Router {
    pub fn new(memory_limit: i32) -> Self {
        Router {
            curr_timestamp: 0,
            memory_limit: memory_limit as usize,
            packages: VecDeque::new(),
            received_pack: HashSet::new(),
            pack_by_desc: HashMap::new(),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        assert!(timestamp >= self.curr_timestamp);

        if timestamp > self.curr_timestamp {
            self.received_pack.clear();
            self.curr_timestamp = timestamp;
        }

        let pack = Package {
            source,
            destination,
            timestamp,
        };

        if !self.received_pack.insert(pack) {
            return false;
        }

        if self.packages.len() == self.memory_limit {
            self.forward_packet();
        }

        self.packages.push_front(pack);
        self.pack_by_desc
            .entry(pack.destination)
            .or_default()
            .push_front(pack);

        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        let Some(first_pack) = self.packages.pop_back() else {
            return vec![];
        };

        if self.curr_timestamp == first_pack.timestamp {
            self.received_pack.remove(&first_pack);
        }

        let desc = self
            .pack_by_desc
            .get_mut(&first_pack.destination)
            .expect("missing desc");
        let first_pack_from_desc = desc.pop_back().expect("missing pack");
        assert_eq!(first_pack, first_pack_from_desc);

        vec![
            first_pack.source,
            first_pack.destination,
            first_pack.timestamp,
        ]
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let Some(desc_queue) = self.pack_by_desc.get(&destination) else {
            return 0;
        };

        desc_queue
            .iter()
            .filter(|p| p.timestamp >= start_time && p.timestamp <= end_time)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut router = Router::new(3);

        assert!(router.add_packet(1, 4, 90));
        assert!(router.add_packet(2, 5, 90));
        assert!(!router.add_packet(1, 4, 90));
        assert!(router.add_packet(3, 5, 95));
        assert!(router.add_packet(4, 5, 105));
        assert_eq!(vec![2, 5, 90], router.forward_packet());
        assert!(router.add_packet(5, 2, 110));
        assert_eq!(1, router.get_count(5, 100, 110));
    }
}
