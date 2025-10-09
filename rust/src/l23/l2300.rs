pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut spells = spells
        .into_iter()
        .enumerate()
        .map(|(i, s)| Spell { power: s, index: i })
        .collect::<Vec<_>>();

    spells.sort_unstable_by_key(|v| v.power);

    let mut potions = potions;
    potions.sort_unstable();

    let mut result = vec![0; spells.len()];

    let mut potion = potions.len();
    for spell in spells {
        while potion > 0 && spell.power as i64 * potions[potion - 1] as i64 >= success {
            potion -= 1;
        }

        result[spell.index] = (potions.len() - potion) as i32;
    }

    result
}

struct Spell {
    power: i32,
    index: usize,
}

#[cfg(test)]
mod tests {
    use crate::parameter_tests;
    use super::*;

    struct Case {
        spells: Vec<i32>,
        potions: Vec<i32>,
        success: i64,
        result: Vec<i32>,
    }

    fn test(c: Case) {
        assert_eq!(c.result, successful_pairs(c.spells, c.potions, c.success))
    }

    parameter_tests!{
        test,
        (case1: Case {
            spells: vec![5, 1, 3],
            potions: vec![1, 2, 3, 4, 5],
            success: 7,
            result: vec![4, 0, 3],
        }),
        (case2: Case {
            spells: vec![3,1,2],
            potions: vec![8,5,8],
            success: 16,
            result: vec![2,0,2],
        }),
    }
}
