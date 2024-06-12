pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    board
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &v)| (i, j, v)))
        .map(|(i, j, v)| {
            if v == '.' {
                return 0;
            }

            if (i == 0 || board[i - 1][j] == '.') && (j == 0 || board[i][j - 1] == '.') {
                return 1;
            }

            0
        })
        .sum::<i32>()
}
