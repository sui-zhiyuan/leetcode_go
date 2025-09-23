use std::str::FromStr;

pub fn compare_version(version1: String, version2: String) -> i32 {
    let version1 = version1
        .split('.')
        .map(|v| i32::from_str(v).unwrap())
        .collect::<Vec<i32>>();

    let version2 = version2
        .split('.')
        .map(|v| i32::from_str(v).unwrap())
        .collect::<Vec<i32>>();

    for i in 0..version1.len().max(version2.len()) {
        let v1 = version1.get(i).copied().unwrap_or(0);
        let v2 = version2.get(i).copied().unwrap_or(0);

        match v1.cmp(&v2) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        }
    }

    0
}
