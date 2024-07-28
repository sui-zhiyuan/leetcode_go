pub fn minimum_operations(num: String) -> i32 {
    let targets = [00, 25, 50, 75];
    let num = num
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .collect::<Vec<_>>();

    let mut remove = i32::MAX;
    for t in targets {
        let th = t / 10;
        let tl = t % 10;

        let pl = num.iter().rposition(|&c| c ==tl);
        if pl.is_none() {
            continue;
        }
        let pl = pl.unwrap();
        let ph = num[..pl].iter().rposition(|&c| c == th);
        if ph.is_none() {
            continue;
        }
        let ph = ph.unwrap();

        remove = remove.min((num.len() - ph - 2) as i32);
    }

    if remove == i32::MAX {
        num.len() as i32 - if num.contains(&0) { 1 } else { 0 }
    } else {
        remove
    }
}
