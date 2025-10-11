use std::collections::HashMap;

pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
    let spells = power
        .into_iter()
        .fold(HashMap::<i32, Spell>::new(), |mut acc, x| {
            let spell = acc.entry(x).or_insert(Spell {
                power: x,
                total_damage: 0,
            });
            spell.total_damage += x as i64;
            acc
        });

    let mut spells = spells.into_values().collect::<Vec<_>>();
    spells.sort_by_key(|v| v.power);

    let mut dp = vec![0i64; spells.len()];

    for (i, spell) in spells.iter().enumerate() {
        let mut taken_power = None;
        let mut max_prev = 0;
        for j in (0..i).rev() {
            if taken_power
                .map(|v| spells[j].power + 3 < v)
                .unwrap_or(false)
            {
                break;
            }

            if spells[j].power + 2 < spell.power {
                max_prev = max_prev.max(dp[j]);
                if taken_power.is_none() {
                    taken_power = Some(spells[j].power);
                }
            }
        }
        dp[i] = max_prev + spell.total_damage;
    }

    dp.into_iter().max().unwrap()
}

struct Spell {
    power: i32,
    total_damage: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        power: Vec<i32>,
        expected_damage: i64,
    }

    fn test(c: Case) {
        assert_eq!(c.expected_damage, maximum_total_damage(c.power))
    }

    parameter_tests! {
        test,
        (case1:Case {
            power: vec![1,1,3,4],
            expected_damage: 6,
        }),
        (case2:Case {
            power: vec![7,1,6,6],
            expected_damage: 13,
        }),
    }
}
