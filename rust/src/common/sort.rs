// cspell:ignore bitonic

use std::cmp::Ordering;

pub fn bitonic_sort<T: Ord>(arr: &mut [T], ord: Ordering) {
    if arr.len() <= 1 {
        return;
    }
    assert!(
        arr.len().is_power_of_two(),
        "{} is not power of 2",
        arr.len()
    );
    let d = arr.len() / 2;
    bitonic_sort(&mut arr[0..d], ord);
    bitonic_sort(&mut arr[d..], ord.reverse());
    bitonic_merge(arr, ord);
}

fn bitonic_merge<T: Ord>(arr: &mut [T], ord: Ordering) {
    if arr.len() <= 1 {
        return;
    }

    let d = arr.len() / 2;
    for i in 0..d {
        if arr[i].cmp(&arr[i + d]) == ord {
            arr.swap(i, i + d);
        }
    }
    bitonic_merge(&mut arr[0..d], ord);
    bitonic_merge(&mut arr[d..], ord);
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};

    use super::*;

    #[test]
    fn test1() {
        let mut arr = vec![3, 7, 4, 8, 6, 2, 1, 5];
        bitonic_sort(&mut arr, Ordering::Less);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test2() {
        let mut arr = vec![3, 7, 4, 8, 6, 2, 1, 5];
        bitonic_sort(&mut arr, Ordering::Greater);
        assert_eq!(arr, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test3() {
        let len = 1 << 15;
        let mut arr = (0..len).collect::<Vec<u32>>();
        let mut rng = rand::rngs::StdRng::seed_from_u64(1000);
        rng.fill(&mut arr[..]);
        bitonic_sort(&mut arr, Ordering::Greater);
        for v in arr.windows(2) {
            if let [v1, v2] = v {
                assert!(v1 <= v2);
            }else{
                unreachable!()
            }
        }
    }
}
