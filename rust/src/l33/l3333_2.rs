pub fn possible_string_count(word: String, k: i32) -> i32 {
    let word = word.chars().collect::<Vec<_>>();
    let n = word.len();
    let mut k = k as usize;
    if n < k {
        return 0;
    }

    const MOD: i64 = 1_000_000_007;
    let mut cnts = Vec::new();

    let mut ans = 1i64;
    let mut cnt = 0i64;

    for i in 0..word.len() {
        cnt += 1;

        if i == n - 1 || word[i] != word[i + 1] {
            if cnt > 1 {
                if k > 0 {
                    cnts.push(cnt - 1);
                }
                ans = ans * cnt % MOD;
            }
            k -= 1;
            cnt = 0;
        }
    }

    if k <= 0 {
        return ans as i32;
    }
    
    dbg!(k);
    dbg!(&cnts);

    let m = cnts.len();
    let mut f = vec![vec![0i64; k]; m + 1];
    f[0].fill(1);
    let mut s = vec![0; k + 1];
    for i in 0..m {
        for j in 0..k {
            s[j + 1] = (s[j] + f[i][j]) % MOD;
        }

        for j in 0..k {
            f[i + 1][j] = (s[j + 1] - s[j.checked_sub(cnts[i] as usize).unwrap_or(0)]) % MOD;
        }
    }

    ((ans + MOD - f[m][k - 1]) % MOD) as i32
}

#[test]

fn test1() {
    assert_eq!(
        834168507,
        possible_string_count(
            "ggggggggaaaaallsssssaaaaaaaaaiiqqqqqqqqqqbbbbbbbvvfffffjjjjeeeeeefffmmiiiix"
                .to_owned(),
            34
        )
    )
}
