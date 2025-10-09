pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    let mut available_time = vec![0; skill.len()];

    for potion_mana in mana {
        let mut start_time = 0;
        let mut elapsed_time = 0i64;
        for (i, &wizard_skill) in skill.iter().enumerate() {
            start_time = start_time.max(available_time[i] - elapsed_time);
            elapsed_time += wizard_skill as i64 * potion_mana as i64;
        }

        let mut elapsed_time = 0i64;
        for (i, available_time) in available_time.iter_mut().enumerate() {
            elapsed_time += skill[i] as i64 * potion_mana as i64;
            *available_time = start_time + elapsed_time;
        }
    }

    available_time.last().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        skill: Vec<i32>,
        mana: Vec<i32>,
        expect: i64,
    }

    fn test(c: Case) {
        assert_eq!(c.expect, min_time(c.skill, c.mana))
    }

    parameter_tests! {
        test,
        (case1: Case {
            skill: vec![1, 5, 2, 4],
            mana: vec![5, 1, 4, 2],
            expect: 110,
        }),
        (case2: Case {
            skill: vec![1,1,1],
            mana: vec![1,1,1],
            expect: 5,
        }),
        (case3: Case {
            skill: vec![1,2,3,4],
            mana: vec![1,2],
            expect: 21,
        }),
    }
}
