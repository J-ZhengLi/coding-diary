mod test;

use calculator_util::ExprParser;
use std::io::{self, Write};

fn main() {
    println!(
        "================================================================================ \
        \n                       This is a very simple calculator \
        \n                                 \x1b[36;1mSupporting:\x1b[0m\
        \n[+]: Addition    [-]: Substrction    [*]: Multiplication    [/]: Division \
        \n[^]: Pow                     [sqrt()]: Square Root      [abs()]: Absolute \
        \n[cos()]: Cosine(Degree)      [sin()]: Sine(Degree)      [tan()]: Tangent(Degree) \
        \n[arccos()]: Arc Cosine       [arcsin()]: Arc Sine       [arctan()]: Arc Tangent \
        \n================================================================================ \
        \n\nEnter an expression to eval. (Ex. `45+(20*3*cos(50)`) \
        \nEnter `exit` to exit the program.\n"
    );

    let mut input = String::new();

    loop {
        print!("[Input]> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read input.");

        match input.trim() {
            "exit" | "e" | "E" => {
                break;
            }
            _ => {
                let exp = input.to_string();
                let res = exp.eval();
                println!("result: {}", res);
            }
        }
        input.clear();
    }
}
