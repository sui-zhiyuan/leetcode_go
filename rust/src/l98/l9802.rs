pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
    let min = *enemy_energies.iter().min().unwrap();
    let sum = enemy_energies.iter().copied().map(|v| v as i64).sum::<i64>();

    if current_energy < min {
        return 0;
    }

    (sum - min as i64 + current_energy as i64) / min as i64
}