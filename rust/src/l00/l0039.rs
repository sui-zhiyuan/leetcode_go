pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates
        .iter()
        .map(|&v| usize::try_from(v).unwrap())
        .collect::<Vec<_>>();
    candidates.sort();
    let target = usize::try_from(target).unwrap();
    let mut state = vec![Node::default(); target + 1];
    state[0].count = 1;
    for &v in candidates.iter() {
        for j in v..=target {
            if state[j - v].count > 0 {
                let count = state[j - v].count;
                state[j].count += count;
                state[j].value.push(Prev { count, value: v });
            }
        }
    }
    
    // dbg!(&state);
    let mut result = vec![vec![]; state[target].count];
    search(&state, &mut result, 0, target , target);

    result
}

fn search(state: &[Node], result: &mut Vec<Vec<i32>>, start: usize, curr: usize, max:usize) {
    if curr == 0 {
        return;
    }
    // dbg!("in");
    let mut start = start;
    for v in state[curr].value.iter() {
        if v.value > max {
            break;
        }
        search(state, result, start, curr - v.value , v.value);
        for rl in result[start..(start + v.count)].iter_mut() {
            rl.push(v.value.try_into().unwrap());
        }
        start += v.count;

        // dbg!(v , &result);
    }
    // dbg!("out");
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    count: usize,
    value: Vec<Prev>,
}

#[derive(Debug, Clone, Default)]
pub struct Prev {
    count: usize,
    value: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        )
    }
}
