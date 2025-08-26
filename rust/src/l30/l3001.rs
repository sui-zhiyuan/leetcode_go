pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    let move1 =
        if a == e && !(a == c && between(b, f, d)) || b == f && !(b == d && between(a, e, c)) {
            1
        } else {
            2
        };

    let move2 = if (c + d) % 2 != (e + f) % 2 {
        i32::MAX
    } else if c + d == e + f && !(a + b == c + d && between(c, e, a))
        || c - d == e - f && !(a - b == c - d && between(c, e, a))
    {
        1
    } else {
        2
    };
    move1.min(move2)
}

fn between(left: i32, right: i32, mid: i32) -> bool {
    left < right && left < mid && mid < right || right < left && right < mid && mid < left
}
