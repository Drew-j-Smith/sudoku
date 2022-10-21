use crate::sudoku::{Position, Sudoku, SudokuTile};

mod sudoku;
mod sudoku2;

fn main() {
    let x = sudoku2::Sudoku::new();
    println!("{x:?}");
    test2();
}

fn simple_random() {
    let mut x = Sudoku::empty();

    for _ in 0..15 {
        match x.add_random() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
    }
    println!("{}", x);

    loop {
        match x.add_least_entropy() {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        };
    }
    println!("{}", x);
}

fn test1() {
    let mut y = Sudoku::empty();
    y.update_for_new_value(Position { row: 0, col: 4 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 0, col: 5 }, SudokuTile::Six);
    y.update_for_new_value(Position { row: 0, col: 7 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 0, col: 8 }, SudokuTile::Nine);
    y.update_for_new_value(Position { row: 1, col: 0 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 1, col: 2 }, SudokuTile::Three);
    y.update_for_new_value(Position { row: 1, col: 6 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 1, col: 7 }, SudokuTile::Six);
    y.update_for_new_value(Position { row: 2, col: 0 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 2, col: 3 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 2, col: 4 }, SudokuTile::Seven);
    y.update_for_new_value(Position { row: 3, col: 1 }, SudokuTile::Five);
    y.update_for_new_value(Position { row: 3, col: 2 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 3, col: 5 }, SudokuTile::Seven);
    y.update_for_new_value(Position { row: 4, col: 3 }, SudokuTile::Six);
    y.update_for_new_value(Position { row: 4, col: 4 }, SudokuTile::Three);
    y.update_for_new_value(Position { row: 4, col: 5 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 4, col: 6 }, SudokuTile::Seven);
    y.update_for_new_value(Position { row: 4, col: 7 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 4, col: 8 }, SudokuTile::Five);
    y.update_for_new_value(Position { row: 5, col: 5 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 6, col: 2 }, SudokuTile::Seven);
    y.update_for_new_value(Position { row: 6, col: 4 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 6, col: 7 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 6, col: 8 }, SudokuTile::Three);
    y.update_for_new_value(Position { row: 7, col: 0 }, SudokuTile::Five);
    y.update_for_new_value(Position { row: 7, col: 3 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 7, col: 5 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 7, col: 6 }, SudokuTile::Eight);
    y.update_for_new_value(Position { row: 7, col: 8 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 8, col: 0 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 8, col: 7 }, SudokuTile::Nine);
    println!("{}", y);

    let board = y.get_board();
    let mut attempts = 0;

    loop {
        attempts += 1;
        match y.add_least_entropy() {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
                match e {
                    sudoku::SudokuError::NoValidSudokuTile => {
                        y = Sudoku::create_from_board(board);
                    }
                    e => {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
        };
    }
    println!("{}", y);
    println!("{attempts} attempts");
}

fn test2() {
    let mut y = Sudoku::empty();
    y.update_for_new_value(Position { row: 0, col: 0 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 0, col: 1 }, SudokuTile::Five);
    y.update_for_new_value(Position { row: 0, col: 5 }, SudokuTile::Nine);
    y.update_for_new_value(Position { row: 0, col: 8 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 1, col: 4 }, SudokuTile::Eight);
    y.update_for_new_value(Position { row: 1, col: 6 }, SudokuTile::Nine);
    y.update_for_new_value(Position { row: 2, col: 1 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 3, col: 2 }, SudokuTile::Two);
    y.update_for_new_value(Position { row: 3, col: 5 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 4, col: 2 }, SudokuTile::Eight);
    y.update_for_new_value(Position { row: 4, col: 8 }, SudokuTile::Three);
    y.update_for_new_value(Position { row: 5, col: 0 }, SudokuTile::Three);
    y.update_for_new_value(Position { row: 5, col: 1 }, SudokuTile::Four);
    y.update_for_new_value(Position { row: 5, col: 4 }, SudokuTile::Seven);
    y.update_for_new_value(Position { row: 5, col: 7 }, SudokuTile::Five);
    y.update_for_new_value(Position { row: 6, col: 0 }, SudokuTile::One);
    y.update_for_new_value(Position { row: 6, col: 1 }, SudokuTile::Nine);
    y.update_for_new_value(Position { row: 6, col: 5 }, SudokuTile::Eight);
    y.update_for_new_value(Position { row: 6, col: 8 }, SudokuTile::Five);
    y.update_for_new_value(Position { row: 7, col: 2 }, SudokuTile::Three);
    y.update_for_new_value(Position { row: 8, col: 3 }, SudokuTile::Six);
    y.update_for_new_value(Position { row: 8, col: 7 }, SudokuTile::Seven);
    println!("{}", y);

    let board = y.get_board();
    let mut attempts = 0;

    loop {
        attempts += 1;
        match y.add_least_entropy() {
            Ok(_) => {}
            Err(e) => match e {
                sudoku::SudokuError::NoValidSudokuTile => {
                    y = Sudoku::create_from_board(board);
                }
                e => {
                    println!("{:?}", e);
                    break;
                }
            },
        };
    }
    println!("{}", y);
    println!("{attempts} attempts");
}
