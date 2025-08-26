pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    let v = dimensions
        .iter()
        .max_by_key(|v| (v[0] * v[0] + v[1] * v[1], v[0] * v[1]))
        .expect("empty input");

    v[0] * v[1]
}
