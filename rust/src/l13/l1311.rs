use std::collections::{HashMap, HashSet, VecDeque};

pub fn watched_videos_by_friends(
    watched_videos: Vec<Vec<String>>,
    friends: Vec<Vec<i32>>,
    id: i32,
    level: i32,
) -> Vec<String> {
    let id = id as usize;
    let mut access = HashSet::<usize>::new();

    let mut queue = VecDeque::<DfsNode>::new();
    queue.push_back(DfsNode { node: id, level: 0 });
    access.insert(id);

    let mut result = HashMap::<String, i32>::new();

    while let Some(curr) = queue.pop_front() {
        if curr.level == level {
            for v in watched_videos[curr.node].iter() {
                *(result.entry(v.to_owned()).or_default()) += 1;
            }
        }

        if curr.level >= level {
            continue;
        }

        for &next in friends[curr.node].iter() {
            let next = next as usize;
            if !access.contains(&next) {
                queue.push_back(DfsNode {
                    node: next,
                    level: curr.level + 1,
                });
                access.insert(next);
            }
        }
    }

    let mut result = result.into_iter().collect::<Vec<_>>();
    result.sort_by_key(|(name, amount)| (*amount, name.to_owned()));
    result.into_iter().map(|v| v.0).collect()
}

struct DfsNode {
    node: usize,
    level: i32,
}
