#![allow(dead_code)]

use calculator_util::{number::Number, ExprParser};

fn eval_test_common(input: &str, expected: Number) {
    let input_string: String = input.to_string();
    let result: Number = input_string.eval();
    assert_eq!(result, expected);
}

#[test]
fn eval_no_operator() {
    eval_test_common("20", Number::Integer(20));
}

#[test]
fn eval_one_op() {
    eval_test_common("1+1", Number::Integer(2));
}

#[test]
fn eval_normal_op() {
    eval_test_common("1.2+2/5", Number::Float(1.6));
}

#[test]
fn eval_neg() {
    eval_test_common("-2.0 * -5 + (20 - -10) / -2", Number::from(-5));
}

#[test]
fn eval_trig() {
    eval_test_common("cos(60) + sin(30)", Number::from(1));
}

#[test]
fn eval_sp_ops() {
    eval_test_common(
        "2 * tan(45) + 8 ^ 2 - abs(-2 * 3) + cos(sqrt(3600))",
        Number::from(60.5),
    );
}
