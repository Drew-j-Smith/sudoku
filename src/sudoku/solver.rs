use super::*;

impl Sudoku {
    fn get_random_unset(&self) -> Option<Position> {
        if self.unset_positions.is_empty() {
            return None;
        }
        let index = thread_rng().gen_range(0..self.unset_positions.len());
        self.unset_positions.iter().nth(index).map(|x| *x)
    }

    fn get_random_from_states(
        &self,
        states: EnumSet<SudokuTile>,
    ) -> Result<SudokuTile, SudokuError> {
        if states.is_empty() {
            return Result::Err(SudokuError::NoValidSudokuTile);
        }
        let index = rand::thread_rng().gen_range(0..states.len());
        match states.iter().nth(index) {
            Some(x) => Result::Ok(x),
            None => Result::Err(SudokuError::NoValidSudokuTile),
        }
    }

    fn get_random(&self, position: Position) -> Result<SudokuTile, SudokuError> {
        match self.board[position.row][position.col] {
            SudokuTileState::Set(_) => Result::Err(SudokuError::BoardHashMapDisagreement),
            SudokuTileState::Unset(x) => self.get_random_from_states(x),
        }
    }

    pub fn add_random(&mut self) -> Result<SudokuTile, SudokuError> {
        match self.get_random_unset() {
            Some(position) => self.update_random_value(position),
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
            self.update_random_value(pos)
        } else {
            Result::Err(SudokuError::BoardHashMapDisagreement)
        }
    }

    pub fn update_random_value(&mut self, position: Position) -> Result<SudokuTile, SudokuError> {
        self.get_random(position).and_then(|x| {
            self.update_for_new_value(position, x);
            Result::Ok(x)
        })
    }

    pub fn update_for_new_value(&mut self, position: Position, new_state: SudokuTile) {
        self.unset_positions.remove(&position);
        self.board[position.row][position.col] = SudokuTileState::Set(new_state);
        self.update_row(position, new_state);
        self.update_col(position, new_state);
        self.update_cell(position, new_state);
    }

    fn update_single_cell(&mut self, position: Position, new_state: SudokuTile) {
        if let SudokuTileState::Unset(x) = self.board[position.row][position.col] {
            self.board[position.row][position.col] = SudokuTileState::Unset(x - new_state);
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
