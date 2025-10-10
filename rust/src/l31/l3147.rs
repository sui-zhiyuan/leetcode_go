pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let mut max_energy = energy;
    let k = k as usize;

    for i in (0..max_energy.len()).rev() {
        if i + k >= max_energy.len() {
            continue;
        }

        max_energy[i] += max_energy[i + k];
    }

    max_energy.iter().max().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        energy: Vec<i32>,
        k: i32,
        expect: i32,
    }

    fn test(c: Case) {
        assert_eq!(c.expect, maximum_energy(c.energy, c.k))
    }

    parameter_tests! {
        test,
        (case1:  Case {
            energy: vec![5,2,-10,-5,1],
            k: 3,
            expect: 3,
        }),
        (case2:  Case {
            energy: vec![-2,-3,-1],
            k: 2,
            expect: -1,
        }),
        (case3:  Case {
            energy: vec![5,-10,4,3,5,-9,9,-7],
            k: 2,
            expect: 23,
        }),
    }
}
