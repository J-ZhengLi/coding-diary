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
    let test = "(1234 + 2345)/5 - 45.5*arccos(20 +54)".to_string();

    println!("{}", test.remove_spaces().to_postfix(Some(" ")));
    println!("{}", test.add_spaces());
}