use std::collections::{BTreeMap, BTreeSet};

pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
    let rewards = reward_values.into_iter().collect::<BTreeSet<_>>();
    let mut cache = BTreeMap::new();
    cache.insert(0 , true);

    for v in (0..=(*rewards.last().unwrap()*2 -1)).rev(){
        if dfs(&mut cache, &rewards, v){
            return v;
        }
    }
    unreachable!()
}

fn dfs(cache: &mut BTreeMap<i32, bool> , rewards: &BTreeSet<i32>, target:i32) -> bool {
    if let Some(&v) = cache.get(&target) {
        return v;
    }
    for &r in rewards.range((target/2 + 1)..target+1) {
        if dfs(cache, rewards, target - r) {
            cache.insert(target, true);
            return true;
        }
    }
    cache.insert(target, false);
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(max_total_reward(vec![1,1,3,3]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(max_total_reward(vec![1,6,4,3,2]), 11);
    }
}