use super::*;

impl Sudoku {
    pub fn empty() -> Sudoku {
        let mut unset_positions: HashSet<Position> = HashSet::new();
        for row in 0..9 {
            for col in 0..9 {
                unset_positions.insert(Position { row, col });
            }
        }
        Sudoku {
            board: [[SudokuTileState::Unset(EnumSet::all()); 9]; 9],
            unset_positions,
        }
    }

    pub fn is_filled(&self) -> bool {
        for ele in &self.unset_positions {
            if let SudokuTileState::Unset(x) = self.board[ele.row][ele.col] {
                if x.len() > 1 {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn get_board(&self) -> [[Option<SudokuTile>; 9]; 9] {
        self.board.map(|row| {
            row.map(|x| match x {
                SudokuTileState::Set(x) => Some(x),
                SudokuTileState::Unset(_) => None,
            })
        })
    }

    pub fn create_from_board(board: [[Option<SudokuTile>; 9]; 9]) -> Sudoku {
        let mut res = Sudoku::empty();
        for row in 0..9 {
            for col in 0..9 {
                if let Some(x) = board[row][col] {
                    res.update_for_new_value(Position { row, col }, x);
                }
            }
        }
        res
    }
}
