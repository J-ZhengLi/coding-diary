//! # calculator_util
//! 
//! Useful functions and traits for writing a calculator app in Rust.
//! This crate has implementation for `String`, that you can parse mathmatical
//! equation strings to Reverse Polish Notation, aka postfix notation, which can
//! be used for mathmatical evaluation with some provided functions under `operations.rs`.
//! 
//! # Example
//! ```rust
//! use calculator_util::{ExprParser, number::Number};
//! 
//! let equation = "(5+6) * 7".to_string();
//! let result = equation.eval();
//! assert_eq!(result, Number::from(77));
//! println!("{}", result); // 77
//! ```

pub mod number;
pub mod operations;

use number::Number;

pub trait ExprParser {
    /// Remove spaces in a `String`
    ///
    /// # Example
    /// ```rust
    /// use calculator_util::ExprParser;
    ///
    /// let expr: String = "abs ( 2 - 13   ) * 5".to_string();
    /// assert_eq!(expr.remove_spaces(), "abs(2-13)*5".to_string());
    /// ```
    fn remove_spaces(&self) -> String;

    /// Add spaces between operators and operands
    ///
    /// Note: This method doesn't work well with Strings that are in postfix notation,
    /// because it's impossible to know where to insert spaces between numbers,
    /// make sure to add a seperator when generating postfix notation using `to_postfix(sep)`.
    ///
    /// # Example
    /// ```rust
    /// use calculator_util::ExprParser;
    ///
    /// let expr: String = "(1+2*3)/3".to_string();
    /// assert_eq!(expr.add_spaces(), "( 1 + 2 * 3 ) / 3".to_string());
    /// ```
    fn add_spaces(&self) -> String;

    /// Convert math expression to a `Vec` of operator and operands in the order of
    /// Reverse Polish Notation (AKA postfix notation)
    ///
    /// This function convert current math expression as a `String` to a `Vec<String>`
    /// which is a serial operators and operands in postfix fashion.
    ///
    /// # Example
    /// ```rust
    /// use calculator_util::ExprParser;
    ///
    /// let infix: String = "3*abs(2-4)".to_string();
    /// let postfix_vec: Vec<String> = infix.to_postfix_vec();
    /// assert_eq!(postfix_vec, vec!["3", "2", "4", "-", "abs", "*"]);
    /// ```
    fn to_postfix_vec(&self) -> Vec<String>;

    /// Convert math expression to Reverse Polish Notation (AKA postfix notation)
    ///
    /// This function convert current expression as a `String` to a postfix notation `String`
    /// The parameter *seperator* is an Option of `&str` that will be used to seperate
    /// the result String, if `None` is given, the result will not be seperated at all.
    ///
    /// # Example
    /// ```rust
    /// use calculator_util::ExprParser;
    ///
    /// let infix: String = "(1+32)/cos(60)".to_string();
    /// let postfix: String = infix.to_postfix(Some(" "));
    /// assert_eq!(postfix, "1 32 + 60 cos /");
    /// ```
    fn to_postfix(&self, seperator: Option<&str>) -> String;

    /// Evaluate given mathatic equation `string`
    /// 
    /// This function evaluates mathmatic equation to a `Number` type, which is a enum
    /// contains both i32 and f64 datatype represent by `Number::Integer(i32) and
    /// `Number::Float(f64)` respectively.
    /// 
    /// # Example
    /// ```rust
    /// use calculator_util::{ExprParser, number::Number};
    /// 
    /// let infix: String = "20 + (cos(60) + sin(30)) - abs(20+ -1)".to_string();
    /// assert_eq!(infix.eval(), Number::from(2));
    /// ```
    fn eval(&self) -> Number;
}

impl ExprParser for String {
    fn remove_spaces(&self) -> String {
        self.trim().split(' ').collect()
    }

    fn add_spaces(&self) -> String {
        // Add this flag to prevent double spaces when two operator are together
        // such as (5+1)/2, the output will be ( 5 + 1 )  / 2 without checking.
        let mut last_ch_is_operator = false;
        let mut new_string = String::new();
        for c in self.chars() {
            if c == ' ' {
                continue;
            }

            if c.is_ascii_digit() || c == '.' {
                new_string.push(c);
                last_ch_is_operator = false;
            } else if c.is_ascii_alphabetic() {
                new_string.push(c);
                last_ch_is_operator = true;
            } else {
                if !last_ch_is_operator {
                    new_string.push(' ');
                }
                new_string.push(c);
                new_string.push(' ');
                last_ch_is_operator = true;
            }
        }
        new_string.trim().to_string()
    }

    fn to_postfix_vec(&self) -> Vec<String> {
        let mut op_buff = String::new();
        let mut num_buff = String::new();
        let mut result: Vec<String> = vec![];
        let mut op_stack: Vec<String> = vec![];
        let mut prev_is_op: bool = true;

        for c in self.chars() {
            // skip whitespaces first
            if c.is_whitespace() {
                continue;
            }

            // handle numbers, including decimal point
            if c == '.' || c.is_ascii_digit() {
                num_buff.push(c);
                prev_is_op = false;
                continue;
            }

            // since this encountered character is not a part of number
            // push remaining number buffer into result and clear it.
            try_push_and_clear(&mut result, &mut num_buff);

            // handle special operators such as 'abs', 'cos', 'sin', etc.
            if c.is_ascii_alphabetic() {
                op_buff.push(c);
                continue;
            }

            // handle paramphesis and the rest of operators including '+', '-', '*', etc.
            if c == '(' {
                op_stack.push(c.to_string());
                // push special operator into stack as well if has
                try_push_and_clear(&mut op_stack, &mut op_buff);
                prev_is_op = true;
            } else if c == ')' {
                while let Some(op) = op_stack.pop() {
                    if op == '('.to_string() {
                        break;
                    }
                    result.push(op);
                }
                prev_is_op = false;
            } else {
                // looking for '+' or '-' because they could be positive or negative sign
                // for numbers, skip if it matches
                if (c == '+' || c == '-') && prev_is_op {
                    num_buff.push(c);
                    continue;
                }
                let op_str = c.to_string();
                while let Some(op) = op_stack.last() {
                    if priority(&op_str) > priority(op) {
                        break;
                    }
                    result.push(op_stack.pop().unwrap());
                }
                op_stack.push(op_str);
                prev_is_op = true;
            }
        }
        // Add remaining operands
        try_push_and_clear(&mut result, &mut num_buff);
        // Add remaining operators
        while let Some(op) = op_stack.pop() {
            result.push(op);
        }

        result
    }

    fn to_postfix(&self, seperator: Option<&str>) -> String {
        let sep: &str = match seperator {
            Some(c) => c,
            None => "",
        };
        self.to_postfix_vec().join(sep)
    }

    fn eval(&self) -> Number {
        let pf_vec = self.to_postfix_vec();
        let mut val_stack: Vec<Number> = vec![];

        for pf in pf_vec {
            match pf.as_str() {
                "+" => do_binary_op(&mut val_stack, operations::add),
                "-" => do_binary_op(&mut val_stack, operations::sub),
                "*" => do_binary_op(&mut val_stack, operations::mul),
                "/" | "\\" => do_binary_op(&mut val_stack, operations::div),
                "^" => do_binary_op(&mut val_stack, operations::pow),
                "sqrt" => do_unary_op(&mut val_stack, operations::sqrt),
                "abs" => do_unary_op(&mut val_stack, operations::abs),
                "cos" => do_unary_op(&mut val_stack, operations::cos),
                "sin" => do_unary_op(&mut val_stack, operations::sin),
                "tan" => do_unary_op(&mut val_stack, operations::tan),
                "arccos" => do_unary_op(&mut val_stack, operations::acos),
                "arcsin" => do_unary_op(&mut val_stack, operations::asin),
                "arctan" => do_unary_op(&mut val_stack, operations::atan),
                _ => {
                    if pf.contains('.') {
                        let parsed_float = pf.parse::<f64>();
                        match parsed_float {
                            Ok(res) => {
                                val_stack.push(Number::Float(res));
                            }
                            Err(_) => {
                                panic!("Fail to parse {:?} as float, please check your input.", pf);
                            }
                        }
                    } else {
                        let parsed_int = pf.parse::<i32>();
                        match parsed_int {
                            Ok(res) => {
                                val_stack.push(Number::Integer(res));
                            }
                            Err(_) => {
                                panic!(
                                    "Fail to parse {:?} as integer, please check your input.",
                                    pf
                                );
                            }
                        }
                    }
                }
            }
        }
        val_stack.pop().unwrap()
    }
}

fn priority(s: &String) -> u8 {
    if s == "+" || s == "-" {
        1
    } else if s == "*" || s == "\\" || s == "/" {
        2
    } else if s == "^" {
        3
    } else {
        0
    }
}

fn try_push_and_clear(seq: &mut Vec<String>, item: &mut String) {
    if !item.is_empty() {
        seq.push(item.clone());
        item.clear();
    }
}

fn do_unary_op(stack: &mut Vec<Number>, op: fn(Option<Number>) -> Number) {
    let res: Number = op(stack.pop());
    stack.push(res);
}

fn do_binary_op(stack: &mut Vec<Number>, op: fn(Option<Number>, Option<Number>) -> Number) {
    let right: Option<Number> = stack.pop();
    let left: Option<Number> = stack.pop();
    let res: Number = op(left, right);
    stack.push(res);
}