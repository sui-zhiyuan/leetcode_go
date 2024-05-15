pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut result = i32::MIN;

    for i in (energy.len()-k)..energy.len(){
        let mut sum = 0;
        let mut j = i;
        loop {
            sum += energy[j];
            result = result.max(sum);
            if j<k {
                break;
            }
            j -= k;
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let energy = vec![5,2,-10,-5,1];
        let k = 3;
        assert_eq!(maximum_energy(energy, k), 3);
    }

    #[test]
    fn test_2() {
        let energy = vec![-2,-3,-1];
        let k = 2;
        assert_eq!(maximum_energy(energy, k), -1);
    }
}