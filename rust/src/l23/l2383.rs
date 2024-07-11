pub fn min_number_of_hours(initial_energy: i32, initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>) -> i32 {
    let required_energy = energy.iter().sum::<i32>() +1;

    let mut required_experience = 0;
    let mut curr_experience = 0;

    for v in experience {
        if curr_experience <= v{
            let diff = v+1 - curr_experience;
            curr_experience += diff;
            required_experience += diff;
        }
        curr_experience += v
    }

    required_energy.max(initial_energy) - initial_energy + required_experience.max(initial_experience) - initial_experience
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let initial_energy = 5;
        let initial_experience = 3;
        let energy = vec![1,4,3,2];
        let experience = vec![2,6,3,1];
        assert_eq!(min_number_of_hours(initial_energy, initial_experience, energy, experience), 8);
    }
}