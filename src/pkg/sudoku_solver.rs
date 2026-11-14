#[allow(dead_code)]
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut modified_board = [[CellState::Empty; 9]; 9];
    for (i, row) in board.iter().enumerate() {
        modified_board[i] = CellState::from_char_vec(row);
    }
    CellState::solve_sudoku(&mut modified_board);
    for r in 0..9 {
        for c in 0..9 {
            board[r][c] = modified_board[r][c].to_char();
        }
    }
}

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum CellState {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Empty,
}

impl CellState {
    pub fn from_char(c: &char) -> Self {
        use CellState::*;
        match c {
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            _ => Empty,
        }
    }

    pub fn to_char(&self) -> char {
        use CellState::*;
        match self {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            _ => '.',
        }
    }

    pub fn from_char_vec(vec: &Vec<char>) -> [CellState; 9] {
        let mut line = [CellState::Empty; 9];
        vec.into_iter()
            .enumerate()
            .for_each(|(i, x)| line[i] = CellState::from_char(x));
        line
    }

    pub fn first_empty_pos(board: &[[CellState; 9]; 9]) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == CellState::Empty {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn valid_states(
        row: usize,
        column: usize,
        board: &[[CellState; 9]; 9],
    ) -> HashSet<CellState> {
        use CellState::*;
        let mut valids = HashSet::from([One, Two, Three, Four, Five, Six, Seven, Eight, Nine]);
        let (cube_row, cube_column) = (row / 3, column / 3);

        for i in 0..9 {
            valids.remove(&board[row][i]);
            valids.remove(&board[i][column]);
            valids.remove(&board[cube_row * 3 + (i % 3)][cube_column * 3 + (i / 3)]);
        }
        valids
    }

    pub fn check_board(board: &[[CellState; 9]; 9]) -> bool {
        CellState::check_rows_columns(board) && CellState::check_cubes(board)
    }

    pub fn check_rows_columns(board: &[[CellState; 9]; 9]) -> bool {
        use CellState::*;
        let template = HashSet::from([One, Two, Three, Four, Five, Six, Seven, Eight, Nine]);
        for i in 0..9 {
            let mut valid_x = template.clone();
            let mut valid_y = template.clone();
            for j in 0..9 {
                valid_x.remove(&board[i][j]);
                valid_y.remove(&board[j][i]);
            }
            if valid_x.len() != 0 || valid_y.len() != 0 {
                return false;
            }
        }
        true
    }

    pub fn check_cubes(board: &[[CellState; 9]; 9]) -> bool {
        use CellState::*;
        let template = HashSet::from([One, Two, Three, Four, Five, Six, Seven, Eight, Nine]);
        for row in 0..3 {
            for column in 0..3 {
                let (cube_row, cube_column) = (row / 3, column / 3);
                let mut valids = template.clone();
                for i in 0..9 {
                    valids.remove(&board[cube_row * 3 + (i % 3)][cube_column * 3 + (i / 3)]);
                }
                if valids.len() != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve_sudoku(board: &mut [[CellState; 9]; 9]) -> bool {
        if let Some((row, col)) = CellState::first_empty_pos(&board) {
            let valid_states = CellState::valid_states(row, col, &board);
            if valid_states.len() == 0 {
                return false;
            }
            for val in valid_states {
                board[row][col] = val;
                if CellState::solve_sudoku(board) {
                    return true;
                }
                board[row][col] = CellState::Empty;
            }
        }
        CellState::check_board(board)
    }
}