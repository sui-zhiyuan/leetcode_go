pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut max = values[0];

    let mut result = i32::MIN;
    for (i , v) in values.iter().enumerate().skip(1){
        result = result.max(max + *v - i as i32);
        max = max.max(*v + i as i32);
    }
    
    result
}