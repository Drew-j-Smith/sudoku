use crate::sudoku::Sudoku;
use wasm_bindgen::prelude::*;

mod sudoku;

#[wasm_bindgen]
pub fn complete_sudoku(vals: Box<[i32]>) -> Box<[JsValue]> {
    let mut sudoku = match Sudoku::create_from_arr(&vals) {
        Ok(x) => x,
        Err(_) => {
            return Box::new([wasm_bindgen::JsValue::from_str("Invalid array")]);
        }
    };

    let mut attempts = 0;
    while attempts < 1000 {
        match sudoku.add_least_entropy() {
            Ok(_) => {}
            Err(e) => match e {
                sudoku::SudokuError::NoValidSudokuTile => {
                    attempts += 1;
                    sudoku = Sudoku::create_from_arr(&vals).expect("Already validated");
                }
                sudoku::SudokuError::BoardFull => {
                    return Box::new(
                        sudoku
                            .to_array()
                            .map(|val| wasm_bindgen::JsValue::from_f64(val as f64)),
                    );
                }
                sudoku::SudokuError::BoardHashMapDisagreement => {
                    return Box::new([wasm_bindgen::JsValue::from_str("Internal error")]);
                }
            },
        };
    }
    Box::new([wasm_bindgen::JsValue::from_str("Timeout")])
}
