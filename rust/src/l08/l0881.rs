pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort();

    let mut i = 0;
    let mut j = people.len() - 1;
    let mut count = 0;
    let mut result = 0;

    while i <= j {
        let rest = limit - people[j];
        while i < j && people[i] <= rest {
            count += 1;
            i += 1;
        }
        if count > 0 {
            count -= 1;
        }
        result += 1;
        if j == 0 {
            break;
        }
        j -= 1;
    }

    result + count / 2 + count % 2
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        let people = vec![3,2,3,2,2];
        let limit = 6;
        let result = num_rescue_boats(people, limit);
        assert_eq!(result, 3);
    }
}