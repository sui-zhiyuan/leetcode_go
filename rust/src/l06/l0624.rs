pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min = i32::MAX;
    let mut second_min = i32::MAX;
    let mut min_index = 0;
    let mut max = i32::MIN;
    let mut second_max = i32::MIN;
    let mut max_index = 0;
    for (i ,r ) in arrays.into_iter().enumerate() {
        if min > r[0] {
            second_min = min;
            min = r[0];
            min_index = i;
        } else if second_min > r[0] {
            second_min = r[0];
        }

        if max < r[r.len() - 1] {
            second_max = max;
            max = r[r.len() - 1];
            max_index = i;
        } else if second_max < r[r.len() - 1] {
            second_max = r[r.len() - 1];
        } 
    }

    if min_index != max_index {
        max - min
    } else {
        (max - second_min).max(second_max - min)
    }
}