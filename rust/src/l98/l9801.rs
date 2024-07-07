pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let mut result =0;
    for (i , &v) in colors.iter().enumerate() {
        let prev = if i == 0 { colors.len() - 1 } else { i - 1 };
        let next = if i == colors.len() - 1 { 0 } else { i + 1 };
        if colors[prev] != v && colors[next] != v {
            result += 1;
        }
    }

    result
}