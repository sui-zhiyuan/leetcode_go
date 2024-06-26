pub fn next_greater_element(n: i32) -> i32 {
    let mut digs = n.to_string().chars().collect::<Vec<_>>();

    let mut values: Vec<(usize, char)> = Vec::new();

    let mut switch = (0, 0);

    for (i, &v) in digs.iter().enumerate().rev() {
        if let Some(&(j, _)) = values.iter().find(|&&(_, u)| u > v) {
            switch = (i, j);
            break;
        }
        values.push((i, v));
    }

    if switch == (0, 0) {
        return -1;
    }

    digs.swap(switch.0, switch.1);
    digs[switch.1 + 1..].sort_unstable();

    digs.iter().collect::<String>().parse().unwrap_or(-1)
}
