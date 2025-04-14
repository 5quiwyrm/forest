use std::fmt;
mod forest_runtime;
use forest_runtime::*;

fn main() {
    use forest_runtime::ForestInstruction::*;
    use forest_runtime::ForestValue::*;
    let mut runtime = forest_runtime::ForestRuntime::new(&[
        Push(Int(0)),
        Push(Int(2)),
        Push(Int(1)),
        Eq,
        If,
        /**/ Drop,
        /**/ Push(Int(0)),
        /******/ If,
        /******/ Push(String("Equal!!!".to_string())),
        /******/ Print,
        /******/ Exit,
        /**/ IfEnd,
        IfEnd,
        Push(String("Not equal!!!!!".to_string())),
        Print,
        Exit,
    ]);
    loop {
        // match runtime.dump() {
        //     Ok(_) => {}
        //     Err(_) => break,
        // }
        match runtime.step() {
            Ok(_) => {}
            Err(e) => {
                match e {
                    ForestError::Halt => {}
                    _ => {
                        eprintln!("{}", e);
                    }
                };
                break;
            }
        };
    }
    // println!("Hello, world!");
}
