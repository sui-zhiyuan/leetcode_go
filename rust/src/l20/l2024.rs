use crate::common::binary_search;

pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let answer_key = answer_key.chars().collect::<Vec<_>>();
    let k = k as usize;
    let mut sum_t = vec![0usize; answer_key.len() + 1];
    for (i, &v) in answer_key.iter().enumerate() {
        sum_t[i + 1] = sum_t[i] + (v == 'T') as usize;
    }

    let mut result = 0;
    for start in 0..answer_key.len() {
        let end1 = binary_search(start, answer_key.len() + 1, |end| {
            sum_t[end] - sum_t[start] > k
        });
        let end2 = binary_search(start, answer_key.len() + 1, |end| {
            (end - start) - (sum_t[end] - sum_t[start]) > k
        });

        result = result.max(end1 - 1 - start).max(end2 - 1 - start);
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // cspell:ignore TTFF
        let answer_key = "TTFF".to_string();
        let k = 2;
        let result = max_consecutive_answers(answer_key, k);
        assert_eq!(result, 4);
    }
}
