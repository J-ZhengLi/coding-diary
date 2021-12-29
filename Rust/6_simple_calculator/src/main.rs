use calculator_util::ExprParser;

fn main() {
    println!(
        "================================================================================ \
        \n                       This is a very simple calculator \
        \n                                 \x1b[36;1mSupporting:\x1b[0m\
        \n[+]: Addition    [-]: Substrction    [*]: Multiplication    [/]: Division \
        \n[^]: Pow                     [sqrt()]: Square Root      [abs()]: Absolute \
        \n[cos()]: Cosine(Degree)      [sin()]: Sine(Degree)      [tan()]: Tangent(Degree) \
        \n[arccos()]: Arc Cosine       [arcsin()]: Arc Sine       [arctan()]: Arc Tangent \
        \n================================================================================"
    );

    // testing
    let test = "(arccos(0.5) + 20 + -10) * (2+8)".to_string();

    println!("Original: {}", test.add_spaces());
    println!("Postfix: {}", test.to_postfix(Some(" ")));
    println!("Result : {}", test.eval());
}
