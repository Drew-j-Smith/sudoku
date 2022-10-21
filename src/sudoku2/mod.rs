use enum_map::{Enum, EnumMap};
use std::collections::HashMap;
use Value::*;

#[derive(Enum, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
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

impl Value {
    const VALUES: [Value; 9] = [One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
    const LOWER_VALUES: [Value; 3] = [One, Two, Three];
    const MIDDLE_VALUES: [Value; 3] = [Four, Five, Six];
    const UPPER_VALUES: [Value; 3] = [Seven, Eight, Nine];
    fn get_box(value: Value) -> &'static [Value; 3] {
        if (value as u8) < 3 {
            &Value::LOWER_VALUES
        } else if (value as u8) < 6 {
            &Value::MIDDLE_VALUES
        } else {
            &Value::UPPER_VALUES
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: Value,
    col: Value,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Set(Value),
    Unset(EnumMap<Value, Option<u8>>),
}

#[derive(Debug)]
struct Move {
    position: Position,
    value: Value,
}

#[derive(Debug)]
pub struct Sudoku {
    tiles: HashMap<Position, Tile>,
    sorted_possible_moves: Vec<Move>,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut tiles: HashMap<Position, Tile> = HashMap::new();
        let mut sorted_possible_moves: Vec<Move> = Vec::new();
        sorted_possible_moves.reserve(Value::VALUES.len().pow(3));
        for row in Value::VALUES {
            for col in Value::VALUES {
                let position = Position { row, col };
                tiles.insert(position, Tile::Unset(EnumMap::default()));
                for value in Value::VALUES {
                    sorted_possible_moves.push(Move { position, value });
                }
            }
        }
        let mut ret = Sudoku {
            tiles,
            sorted_possible_moves,
        };
        for row in Value::VALUES {
            for col in Value::VALUES {
                for value in Value::VALUES {
                    let position = Position { row, col };
                    let entropy = ret.calculate_entropy(position, value);
                    if let Tile::Unset(map) = ret.tiles.get_mut(&position).unwrap() {
                        map[value] = Some(entropy);
                    }
                }
            }
        }
        ret.sort();
        ret
    }

    fn sort(&mut self) {
        self.sorted_possible_moves.sort_by_key(|mov| {
            self.tiles
                .get(&mov.position)
                .and_then(|tile| match tile {
                    Tile::Set(_) => None,
                    Tile::Unset(map) => map[mov.value],
                })
                .or(None)
        });
    }

    fn get_entropy(&self, position: Position, value: Value) -> Option<u8> {
        self.tiles.get(&position).and_then(|tile| match tile {
            Tile::Set(_) => None,
            Tile::Unset(map) => map[value],
        })
    }

    fn fold_row<F, T>(row: Value, f: F, init: T) -> T
    where
        F: FnMut(T, Position) -> T,
    {
        Value::VALUES
            .iter()
            .map(|col| Position { row, col: *col })
            .fold(init, f)
    }

    fn fold_col<F, T>(col: Value, f: F, init: T) -> T
    where
        F: FnMut(T, Position) -> T,
    {
        Value::VALUES
            .iter()
            .map(|row| Position { row: *row, col })
            .fold(init, f)
    }

    fn fold_box<F, T>(position: Position, f: F, init: T) -> T
    where
        F: Fn(T, Position) -> T,
        F: Copy,
    {
        Value::get_box(position.row).iter().fold(init, |init, row| {
            Value::get_box(position.col)
                .iter()
                .map(|col| Position {
                    row: *row,
                    col: *col,
                })
                .fold(init, f)
        })
    }

    fn calculate_entropy(&self, position: Position, value: Value) -> u8 {
        let moves: u8 = match self.tiles.get(&position).unwrap() {
            Tile::Set(_) => 0,
            Tile::Unset(map) => map.iter().fold(0, |accum, item| match item.1 {
                Some(_) => accum + 1,
                None => accum,
            }),
        };
        let f = |init, position| match self.get_entropy(position, value) {
            Some(_) => init + 1,
            None => init,
        };
        let moves = Sudoku::fold_row(position.row, f, moves);
        let moves = Sudoku::fold_col(position.col, f, moves);
        Sudoku::fold_box(position, f, moves)
    }
}
