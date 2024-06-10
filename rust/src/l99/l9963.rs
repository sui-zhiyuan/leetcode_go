use std::collections::BTreeSet;

pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
    let rewards = reward_values.into_iter().collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
    let mut result = BTreeSet::new();
    result.insert(0);
    let mut max = 0;

    for v in rewards{
        let mut new_values = Vec::new();
        for r in result.iter().copied() {
            if r >= v {
                break;
            }
            new_values.push(r+v);
        }
        for nv in new_values {
            max= max.max(nv);
            result.insert(nv);
        }
    }
    max
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