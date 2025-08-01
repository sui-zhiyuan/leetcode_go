pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut result = Vec::new();
    for i in 1..=num_rows {
        let mut row = Vec::with_capacity(i);
        let prev: &Vec<i32> = if i >= 2 { &result[i - 2]} else {&vec![]};
        for j in 0..i {
            if j == 0 || j == i - 1 {
                row.push(1);
            } else {
                row.push(prev[j - 1] + prev[j])
            }
        }
        result.push(row);
    }
    result
}
