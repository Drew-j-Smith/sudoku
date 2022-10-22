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

    pub fn create_from_arr(arr: &Box<[i32]>) -> Result<Sudoku, ()> {
        if arr.len() != 81 {
            return Err(());
        }
        let mut res = Sudoku::empty();
        for row in 0..9 {
            for col in 0..9 {
                if let Some(val) = num::FromPrimitive::from_i32(arr[(row * 9) + col]) {
                    res.update_for_new_value(Position { row, col }, val);
                }
            }
        }
        Ok(res)
    }

    pub fn to_array(&self) -> [i32; 81] {
        let mut res = [-1; 81];
        for row in 0..9 {
            for col in 0..9 {
                if let SudokuTileState::Set(x) = self.board[row][col] {
                    res[(row * 9) + col] = x as i32;
                }
            }
        }
        res
    }
}
