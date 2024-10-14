use std::mem;

pub fn count_winning_sequences(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    const MOD: i64 = 1_000_000_007;

    let mut curr = vec![[0i64; 3]; n * 2 + 1];
    let mut next = vec![[0i64; 3]; n * 2 + 1];
    // state[win_count][last]

    curr[0]= [1,1,1];

    for (round, &enemy) in s.iter().enumerate() {
        let enemy = match enemy {
            'F' => 0,
            'W' => 1,
            'E' => 2,
            _ => unreachable!(),
        };

        for me in 0..=2 {
            let win_add = match ((me + 3) - enemy) % 3 {
                1 => 2, // win
                2 => 0, // lose
                0 => 1, // draw
                _ => unreachable!(),
            };

            if round == 0 {
                next[win_add][me] = 1;
                continue;
            }

            for win_count in 0..=n * 2 {
                next[win_count][me] = 0;

                if win_count < win_add {
                    continue;
                }

                for prev_me in 0..=2 {
                    if prev_me == me {
                        continue;
                    }
                    next[win_count][me] = (next[win_count][me] + curr[win_count - win_add][prev_me]) % MOD;
                }
            }
        }
        mem::swap(&mut curr, &mut next);
    }

    let mut result = 0;
    for i in n + 1..=n * 2 {
        for me in 0..=2 {
            result = (result + curr[i][me])% MOD;
        }
    }

    result as i32
}

// F -> 0
// W -> 1
// E -> 2
// +1 WIN
// -1 LOSE
// alice -> enemy
// bob -> me

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "FFF".to_string();
        assert_eq!(count_winning_sequences(s), 3);
    }

    #[test]
    fn test2() {
        let s = "FWEFW".to_string();
        assert_eq!(count_winning_sequences(s), 18);
    }
}
