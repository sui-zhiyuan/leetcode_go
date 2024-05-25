use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut count = HashMap::<i32, (i32, i32)>::new();

    for v in matches.iter().map(|v| &v[..]) {
        if let &[winner, loser, ..] = v {
            count.entry(winner).or_insert((0, 0)).0 += 1;
            count.entry(loser).or_insert((0, 0)).1 += 1;
        }
    }

    let mut win_all = count.iter().filter(|v| v.1.1 == 0).map(|v| *v.0).collect::<Vec<_>>();
    let mut lose1 = count.iter().filter(|v| v.1.1 == 1).map(|v| *v.0).collect::<Vec<_>>();
    win_all.sort();
    lose1.sort();
    vec![win_all , lose1]
}
