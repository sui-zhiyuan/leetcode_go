use crate::common::binary_search;

pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    let total_dist = dist.iter().map(|v| *v as f64).sum::<f64>();
    let n = dist.len() as i32;

    if hour <= (n - 1) as f64 {
        return -1;
    }

    let min_speed = (total_dist / hour).trunc() as i32;
    let max_speed = (total_dist / (hour - (n as f64 - 1.0)))
        .ceil()
        .min(10_000_001.0) as i32;

    binary_search(min_speed, max_speed, |speed| {
        time_taken(&dist, speed) <= hour
    })
}

fn time_taken(dist: &[i32], speed: i32) -> f64 {
    dist.iter()
        .map(|&d| d as f64 / speed as f64)
        .fold(0.0, |acc, x| acc.ceil() + x)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test1() {
        let dist = vec![1, 3, 2];
        let hour = 6.0;
        assert_eq!(min_speed_on_time(dist, hour), 1);
    }

    #[test]
    fn test2() -> Result<(), Box<dyn std::error::Error>> {
        let path = env::current_dir()?;

        println!("The current directory is {}", path.display());

        let dist = std::fs::read_to_string(path.join("src/l18/l1870_case2.txt"))?;
        let dist = dist.trim_start_matches('[').trim_end_matches(']').split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let hour = 842738137.43;
        assert_eq!(min_speed_on_time(dist, hour), 6);
        Ok(())
    }
}
