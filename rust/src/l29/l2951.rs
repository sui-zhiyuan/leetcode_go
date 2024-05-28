pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    mountain
        .windows(3)
        .enumerate()
        .filter(|(_, w)| w[0] < w[1] && w[1] > w[2])
        .map(|(i, _)| (i + 1) as i32)
        .collect()
}
