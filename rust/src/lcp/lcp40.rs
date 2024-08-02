use std::cmp;

// cspell:ignore maxmium_score
pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
    let mut cards = cards;
    cards.sort_by_key(|v| cmp::Reverse(*v));

    let cnt = cnt as usize;

    let sum = cards.iter().take(cnt).sum::<i32>();
    if sum % 2 == 0 {
        return sum;
    }

    let mut result = 0;

    let minimum_even = cards.iter().take(cnt).filter(|v| **v % 2 == 0).min();
    let maximum_replace_odd = cards.iter().skip(cnt).filter(|v| **v % 2 == 1).max();

    if let (Some(minimum_even), Some(maximum_replace_odd)) = (minimum_even, maximum_replace_odd) {
        result = result.max(sum - minimum_even + maximum_replace_odd);
    }

    let minimum_odd = cards.iter().take(cnt).filter(|v| **v % 2 == 1).min();
    let maximum_replace_even = cards.iter().skip(cnt).filter(|v| **v % 2 == 0).max();
    if let (Some(minimum_odd), Some(maximum_replace_even)) = (minimum_odd, maximum_replace_even) {
        result = result.max(sum - minimum_odd + maximum_replace_even);
    }

    result
}
