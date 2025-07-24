use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};

pub fn possible_string_count(word: String, k: i32) -> i32 {
    let mut char_counts = Vec::new();
    let mut curr_char = '-';

    for c in word.chars() {
        if c != curr_char {
            char_counts.push(1usize);
            curr_char = c;
            continue;
        }
        *char_counts.last_mut().unwrap() += 1;
    }

    // at least keep 1 repeat for each char
    for v in char_counts.iter_mut() {
        *v -= 1;
    }

    let miss_char_count = word.len().checked_sub(k as usize).unwrap();

    let mut count = vec![ModInteger::from(1); miss_char_count + 1];
    let mut sum = vec![ModInteger::from(0); miss_char_count + 1];

    dbg!(miss_char_count);
    dbg!(&char_counts);

    for &curr_count in char_counts.iter() {
        sum[0] = count[0];
        for j in 1..sum.len() {
            sum[j] = sum[j - 1] + count[j]
        }

        for (j, v) in count.iter_mut().enumerate() {
            *v = if j <= curr_count {
                sum[j]
            } else {
                sum[j] - sum[j - curr_count - 1]
            }
        }
    }

    count[miss_char_count].into()
}

#[derive(Copy, Clone)]
struct ModInteger(i32);

const MOD: i32 = 1_000_000_009;

impl Add for ModInteger {
    type Output = ModInteger;

    fn add(self, rhs: Self) -> Self::Output {
        ModInteger((self.0 + rhs.0) % MOD)
    }
}

impl Sub for ModInteger {
    type Output = ModInteger;

    fn sub(self, rhs: Self) -> Self::Output {
        ModInteger((self.0 + MOD - rhs.0) % MOD)
    }
}

impl From<i32> for ModInteger {
    fn from(value: i32) -> Self {
        ModInteger(value)
    }
}

impl Into<i32> for ModInteger {
    fn into(self) -> i32 {
        self.0
    }
}

impl Debug for ModInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(5, possible_string_count("aabbccdd".to_owned(), 7))
    }

    #[test]
    fn test2() {
        assert_eq!(1, possible_string_count("aabbccdd".to_owned(), 8))
    }

    #[test]
    fn test3() {
        assert_eq!(8, possible_string_count("aaabbb".to_owned(), 3))
    }

    #[test]
    fn test4() {
        assert_eq!(
            834168507,
            possible_string_count(
                "ggggggggaaaaallsssssaaaaaaaaaiiqqqqqqqqqqbbbbbbbvvfffffjjjjeeeeeefffmmiiiix"
                    .to_owned(),
                34
            )
        )
    }
}
