pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let pattern = pattern.chars().collect::<Vec<_>>();
    let [left , right] = pattern[..] else {
        panic!("Invalid input");
    };

    let mut count_left = 0;
    let mut count_right = 0;
    let mut result = 0;

    for c in text.chars() { 
        if c == right {
            result += count_left;
            count_right += 1;
        }
        if c == left {
            count_left += 1;
        }
    }

    result + count_left.max(count_right)
}