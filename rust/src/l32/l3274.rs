pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    let c1x = coordinate1.chars().next().unwrap() as i32 - 'a' as i32;
    let c1y = coordinate1.chars().nth(1).unwrap() as i32 - '1' as i32;
    let c2x = coordinate2.chars().next().unwrap() as i32 - 'a' as i32;
    let c2y = coordinate2.chars().nth(1).unwrap() as i32 - '1' as i32;

    (c1x + c1y) % 2 == (c2x + c2y) % 2
}