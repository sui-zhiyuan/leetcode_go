pub fn longest_continuous_substring(s: String) -> i32 {
    let mut result =0;
    let mut starrt = 0;
    let mut diff = 0;

    for (i , c) in s.chars().enumerate(){
        let c_diff = i as i32 - c as i32;
        if c_diff != diff {
            diff = c_diff;
            starrt = i;
        }
        result = result.max(i - starrt + 1);
    }

    result as i32
}