pub fn min_changes(n: i32, k: i32) -> i32 {
    if n & k != k {
        return -1;
    }

    let dif = n & !k;
    dif.count_ones() as i32
}