use crate::sodoku::Sodoku;

mod sodoku;

fn main() {
    let mut x = Sodoku::empty();
    for _ in 0..81 {
        x.add_random();
    }
    println!("{:?}", x);
}
