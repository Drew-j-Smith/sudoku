use super::*;

impl Sudoku {
    fn get_random_unset(&self) -> Option<Position> {
        if self.unset_positions.is_empty() {
            return Option::None;
        }
        let index = rand::thread_rng().gen_range(0..self.unset_positions.len());
        match self.unset_positions.iter().nth(index) {
            Some(position) => Option::Some((*position).clone()),
            None => Option::None,
        }
    }

    fn set_random_from_states(
        &mut self,
        position: Position,
        states: EnumSet<SudokuTile>,
    ) -> Result<SudokuTile, SudokuError> {
        if states.is_empty() {
            return Result::Err(SudokuError::NoValidSudokuTile);
        }
        let index = rand::thread_rng().gen_range(0..states.len());
        match states.iter().nth(index) {
            Some(x) => {
                self.board[position.row][position.col] = SudokuTileState::Set(x);
                Result::Ok(x)
            }
            None => Result::Err(SudokuError::NoValidSudokuTile),
        }
    }

    fn set_random(&mut self, position: Position) -> Result<SudokuTile, SudokuError> {
        match self.board[position.row][position.col] {
            SudokuTileState::Set(_) => Result::Err(SudokuError::BoardHashMapDisagreement),
            SudokuTileState::Unset(x) => self.set_random_from_states(position, x),
        }
    }

    pub fn add_random(&mut self) -> Result<SudokuTile, SudokuError> {
        match self.get_random_unset() {
            Some(position) => {
                self.unset_positions.remove(&position);
                let res = self.set_random(position);
                match res {
                    Ok(new_state) => self.update_for_new_value(position, new_state),
                    Err(_) => {}
                }
                res
            }
            None => Result::Err(SudokuError::BoardFull),
        }
    }

    pub fn add_least_entropy(&mut self) -> Result<SudokuTile, SudokuError> {
        if self.unset_positions.is_empty() {
            return Result::Err(SudokuError::BoardFull);
        }
        let mut pos = self.unset_positions.iter().next().expect("").clone();
        let min = self.board[pos.row][pos.col];
        if let SudokuTileState::Unset(mut min) = min {
            for ele in &self.unset_positions {
                let curr = self.board[ele.row][ele.col];
                if let SudokuTileState::Unset(curr) = curr {
                    if curr.len() < min.len() {
                        min = curr;
                        pos = *ele;
                    }
                }
            }
            let position = pos.clone();
            self.unset_positions.remove(&pos);
            let res = self.set_random(position);
            match res {
                Ok(new_state) => self.update_for_new_value(position, new_state),
                Err(_) => {}
            }
            res
        } else {
            Result::Err(SudokuError::BoardHashMapDisagreement)
        }
    }

    fn update_for_new_value(&mut self, position: Position, new_state: SudokuTile) {
        self.update_row(position, new_state);
        self.update_col(position, new_state);
        self.update_cell(position, new_state);
    }

    fn update_single_cell(&mut self, position: Position, new_state: SudokuTile) {
        match self.board[position.row][position.col] {
            SudokuTileState::Set(_) => {}
            SudokuTileState::Unset(x) => {
                if x.len() == 1 && x != new_state {
                    if let Some(x) = x.iter().next() {
                        self.board[position.row][position.col] = SudokuTileState::Set(x);
                        self.unset_positions.remove(&position);
                        self.update_for_new_value(position, x);
                    }
                } else {
                    self.board[position.row][position.col] = SudokuTileState::Unset(x - new_state);
                }
            }
        }
    }

    fn update_row(&mut self, position: Position, new_state: SudokuTile) {
        let row = position.row;
        for col in 0..9 {
            self.update_single_cell(Position { row, col }, new_state)
        }
    }

    fn update_col(&mut self, position: Position, new_state: SudokuTile) {
        let col = position.col;
        for row in 0..9 {
            self.update_single_cell(Position { row, col }, new_state)
        }
    }

    fn update_cell(&mut self, position: Position, new_state: SudokuTile) {
        fn get_range(index: usize) -> std::ops::Range<usize> {
            match index {
                0..=2 => 0..3,
                3..=5 => 3..6,
                6..=8 => 6..9,
                _ => panic!("invlaid index"),
            }
        }
        for row in get_range(position.row) {
            for col in get_range(position.col) {
                self.update_single_cell(Position { row, col }, new_state)
            }
        }
    }
}
