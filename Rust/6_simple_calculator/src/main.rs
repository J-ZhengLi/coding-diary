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
    let test = "3 - 4*5*cos(60)".to_string();

    println!("{}", test.to_postfix(Some(" ")));
    println!("{}", test.add_spaces());
}