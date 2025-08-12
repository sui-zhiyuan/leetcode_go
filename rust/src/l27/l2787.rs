use std::ops::AddAssign;

pub fn number_of_ways(n: i32, x: i32) -> i32 {
    let x = x as u32;
    let n = n as usize;
    let mut base = 1usize;

    while base.pow(x) <= n {
        base += 1
    }

    let mut values: Vec<ModNumber> = vec![0.into(); n + 1];
    values[0] = 1.into();

    while base > 0 {
        let p_base = base.pow(x);
        if p_base > n {
            base -= 1;
            continue;
        }

        for i in (p_base..=n).rev() {
            let adder = values[i - p_base];
            values[i] += adder;
        }
        base -= 1;
    }

    values[n].0
}

#[derive(Copy, Clone, Debug)]
struct ModNumber(i32);

const MOD: i32 = 1_000_000_007;

impl From<i32> for ModNumber {
    fn from(value: i32) -> Self {
        ModNumber(value)
    }
}

impl AddAssign for ModNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % MOD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(2, number_of_ways(4, 1))
    }
}
