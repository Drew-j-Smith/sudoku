use enumset::EnumSet;
use enumset::EnumSetType;
use rand;
use rand::Rng;
use std::collections::HashSet;
use std::fmt;

#[derive(EnumSetType)]
pub enum SudokuTile {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(Debug)]
pub enum SudokuError {
    NoValidSudokuTile,
    BoardHashMapDisagreement,
    BoardFull,
}

impl std::fmt::Display for SudokuTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Clone, Copy)]
enum SudokuTileState {
    Set(SudokuTile),
    Unset(EnumSet<SudokuTile>),
}

impl std::fmt::Display for SudokuTileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SudokuTileState::Set(x) => x.fmt(f),
            SudokuTileState::Unset(_) => write!(f, " "),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

pub struct Sudoku {
    board: [[SudokuTileState; 9]; 9],
    unset_positions: HashSet<Position>,
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "-------------------\n");
        for row in self.board {
            for ele in row {
                res = res.and(write!(f, "|{}", ele));
            }
            res = res.and(write!(f, "|\n-------------------\n"));
        }
        res
    }
}

fn get_range(index: usize) -> std::ops::Range<usize> {
    match index {
        0..=2 => 0..3,
        3..=5 => 3..6,
        6..=8 => 6..9,
        _ => panic!("invlaid index"),
    }
}

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
        for row in get_range(position.row) {
            for col in get_range(position.col) {
                self.update_single_cell(Position { row, col }, new_state)
            }
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
        Sudoku {
            board: board.map(|row| {
                row.map(|x| match x {
                    Some(x) => SudokuTileState::Set(x),
                    None => SudokuTileState::Unset(EnumSet::empty()),
                })
            }),
            unset_positions: HashSet::new(),
        }
    }
}
