pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    board.iter().enumerate().map(|(y, row)| {
        row.chars().enumerate().map(|(x, col)| {
            if col == '*' {
                return '*';
            }

            let count = count_mines(board, x as i32, y as i32);

            if count > 0 {
                count.to_string().chars().next().unwrap()
            } else {
                ' '
            }
        }).collect()
    }).collect()
}

fn is_mine(board: &Vec<&str>, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || y >= board.len() as i32 {
        return false;
    }

    match board[y as usize].chars().nth(x as usize) {
        Some(c) => c == '*',
        None => false,
    }
}

fn count_mines(board: &Vec<&str>, x: i32, y: i32) -> usize {
    ((x - 1)..(x + 2)).into_iter().map(|cx| {
        ((y - 1)..(y + 2)).into_iter().filter(|&cy| {
            (cx != x || cy != y) && is_mine(board, cx, cy)
        }).count()
    }).sum()
}
