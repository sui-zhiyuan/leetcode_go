use std::cmp;

pub fn minimum_cost(
    _m: i32,
    _n: i32,
    horizontal_cut: Vec<i32>,
    vertical_cut: Vec<i32>,
) -> i64 {
    let mut cuts = horizontal_cut
        .into_iter()
        .map(|v| Cut { value: v, kind: 0 })
        .chain(vertical_cut.into_iter().map(|v| Cut { value: v, kind: 1 }))
        .collect::<Vec<_>>();

    cuts.sort_by_key(|v| cmp::Reverse(v.value));

    let mut result = 0;
    let mut cut0 = 1;
    let mut cut1 = 1;
    for cur in cuts {
        if cur.kind == 0 {
            cut0 += 1;
            result += (cur.value as i64) * cut1;
        } else {
            cut1 += 1;
            result += (cur.value as i64) * cut0;
        }
    }

    result
}

struct Cut {
    value: i32,
    kind: usize,
}
