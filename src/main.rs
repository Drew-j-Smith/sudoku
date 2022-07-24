use crate::sudoku::{Sudoku, SudokuError};

mod sudoku;

fn main() {
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
