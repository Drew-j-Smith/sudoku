use enumset::EnumSet;
use enumset::EnumSetType;
use rand;
use rand::Rng;
use std::collections::HashSet;
use std::fmt;

#[derive(EnumSetType)]
pub enum SodokuTile {
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

pub enum SodokuError {
    NoValidSodokuTile,
    BoardHashMapDisagreement,
    BoardFull,
}

impl fmt::Debug for SodokuTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

#[derive(Clone, Copy)]
pub enum SodokuTileState {
    Set(SodokuTile),
    Unset(EnumSet<SodokuTile>),
}

impl fmt::Debug for SodokuTileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SodokuTileState::Set(x) => x.fmt(f),
            SodokuTileState::Unset(_) => write!(f, " "),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Position {
    row: usize,
    col: usize,
}

pub struct Sodoku {
    board: [[SodokuTileState; 9]; 9],
    unset_positions: HashSet<Position>,
}

impl fmt::Debug for Sodoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "-------------------\n");
        for row in self.board {
            for ele in row {
                res = res.and(write!(f, "|{:?}", ele));
            }
            res = res.and(write!(f, "|\n-------------------\n"));
        }
        res
    }
}

impl Sodoku {
    pub fn empty() -> Sodoku {
        let mut unset_positions: HashSet<Position> = HashSet::new();
        for row in 0..9 {
            for col in 0..9 {
                unset_positions.insert(Position { row, col });
            }
        }
        Sodoku {
            board: [[SodokuTileState::Unset(EnumSet::all()); 9]; 9],
            unset_positions,
        }
    }

    fn get_random_unset(&self) -> Option<Position> {
        let index = rand::thread_rng().gen_range(0..self.unset_positions.len());
        match self.unset_positions.iter().nth(index) {
            Some(position) => Option::Some((*position).clone()),
            None => Option::None,
        }
    }

    fn set_random_from_states(
        &mut self,
        position: Position,
        states: EnumSet<SodokuTile>,
    ) -> Result<(), SodokuError> {
        let index = rand::thread_rng().gen_range(0..states.len());
        match states.iter().nth(index) {
            Some(x) => {
                self.board[position.row][position.col] = SodokuTileState::Set(x);
                Result::Ok(())
            }
            None => Result::Err(SodokuError::NoValidSodokuTile),
        }
    }

    fn set_random(&mut self, position: Position) -> Result<(), SodokuError> {
        match self.board[position.row][position.col] {
            SodokuTileState::Set(_) => Result::Err(SodokuError::BoardHashMapDisagreement),
            SodokuTileState::Unset(x) => self.set_random_from_states(position, x),
        }
    }

    pub fn add_random(&mut self) -> Result<(), SodokuError> {
        match self.get_random_unset() {
            Some(position) => {
                self.unset_positions.remove(&position);
                self.set_random(position)
            }
            None => Result::Err(SodokuError::BoardFull),
        }
    }
}
