pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let m = board.len();
    let n = board[0].len();

    let r_move = r_move as usize;
    let c_move = c_move as usize;

    if board[r_move][c_move] != '.' {
        return false;
    }

    for d in directions {
        let mut r = r_move as i32 + d.0;
        let mut c = c_move as i32 + d.1;
        if r < 0 || r >= m as i32 || c < 0 || c >= n as i32 {
            continue;
        }
        if board[r as usize][c as usize] == color || board[r as usize][c as usize] == '.' {
            continue;
        }
        r += d.0;
        c += d.1;
        while r >= 0 && r < m as i32 && c >= 0 && c < n as i32 {
            if board[r as usize][c as usize] == '.' {
                break;
            }
            if board[r as usize][c as usize] == color {
                return true;
            }
            r += d.0;
            c += d.1;
        }
    }
    false
}
