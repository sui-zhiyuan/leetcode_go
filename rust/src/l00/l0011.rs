use crate::common::binary_search;

pub fn max_area(mut height: Vec<i32>) -> i32 {
    let v1 = find_max_right_smaller(&height);
    height.reverse();
    let v2 = find_max_right_smaller(&height);
    v1.max(v2)
}

fn find_max_right_smaller(height: &[i32]) -> i32 {
    let mut first_monotonic_stack = Vec::new();

    let mut max_area = 0;
    for (i , &v) in height.iter().enumerate() {
        if first_monotonic_stack.is_empty() {
            first_monotonic_stack.push(Edge{index:i, height:v});
            continue;
        } 

        let just_higher = binary_search(0, first_monotonic_stack.len(), |j| {
            first_monotonic_stack[j].height >= v
        });

        if just_higher == first_monotonic_stack.len() {
            first_monotonic_stack.push(Edge{index:i, height:v});
            continue;
        }

        let new_area = (i - first_monotonic_stack[just_higher].index) as i32 * v;
        max_area = max_area.max(new_area);
    }

    max_area
}

struct Edge{
    index:usize,
    height:i32,
}