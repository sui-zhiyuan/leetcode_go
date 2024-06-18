use std::cmp;

// cspell:ignore strs slength
pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
    strs.sort_by_key(|v| cmp::Reverse(v.len()));

    for (i , s) in strs.iter().enumerate() {
        let mut allow = true;
        for (j , ts) in strs.iter().enumerate(){
            if j == i {
                continue;
            }

            if ts.len() < s.len() {
                break;
            }

            if ts.len() == s.len()  && s == ts {
                allow = false;
                break;
            }

            if ts.len() > s.len()  && is_sub_seq(ts , s) {
                allow = false;
                break;
            }
        }

        if allow {
            return s.len() as i32;
        }
    }
    -1
}

fn is_sub_seq(a: &str, b:&str) -> bool{
    let a= a.chars().collect::<Vec<char>>();
    let b= b.chars().collect::<Vec<char>>();

    max_sub_seq(&a, &b) == b.len()
}

fn max_sub_seq(a: &[char], b:&[char]) -> usize{
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i , &ca) in a.iter().enumerate() {
        for (j, &cb) in b.iter().enumerate(){
            if ca == cb {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            }else {
                dp[i + 1][j + 1] = dp[i][j];
            }
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i + 1][j]).max(dp[i][j + 1]);
        }
    }

    dp[a.len()][b.len()]
}