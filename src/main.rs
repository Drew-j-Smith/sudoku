use crate::sudoku::Sudoku;

mod sudoku;

fn main() {
    let mut x = Sudoku::empty();
    for _ in 0..81 {
        x.add_random();
    }
    println!("{}", x);
}
