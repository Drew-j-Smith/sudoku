use enumset::EnumSet;
use enumset::EnumSetType;
use rand;
use rand::thread_rng;
use rand::Rng;
use std::collections::HashSet;
use std::fmt;

mod solver;
mod util;

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
pub struct Position {
    pub row: usize,
    pub col: usize,
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
