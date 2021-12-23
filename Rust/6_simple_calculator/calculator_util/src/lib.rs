pub trait ExprParser {
    /// Remove spaces in a `String`
    /// 
    /// Sometimes you might just want a cleaner look version on your expression,
    /// espacially before passing it to `to_postfix` method
    /// 
    /// # Example
    /// ```rust
    /// use calculator_util::ExprParser;
    /// 
    /// let expr: String = "abs( 2 - 13 ) * 5".to_string();
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
    /// let expr: String = "(1+2*30)/3".to_string();
    /// assert_eq!(expr.add_spaces(), "( 1 + 2 * 30 ) / 3".to_string());
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
    /// let infix: String = "3*(10+2)".to_string();
    /// let postfix_vec: Vec<String> = infix.to_postfix_vec();
    /// assert_eq!(postfix_vec, vec!["3", "10", "2", "+", "*"]);
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
    /// let infix: String = "(1+32)/3".to_string();
    /// let postfix: String = infix.to_postfix(Some(" "));
    /// assert_eq!(postfix, "1 32 + 3 /");
    /// ```
    fn to_postfix(&self, seperator: Option<&str>) -> String;
}

impl ExprParser for String {
    fn remove_spaces(&self) -> String {
        self.trim().split(' ').collect()
    }

    fn add_spaces(&self) -> String {
        // To simplify the whole procedure, remove all existing spaces before hand.
        let old_string = self.remove_spaces();

        // Add this flag to prevent double spaces when two operator are together
        // such as (5+1)/2, the output will be ( 5 + 1 )  / 2 without checking.
        let mut last_ch_is_operator = false;

        let mut new_string = String::new();
        for c in old_string.chars() {
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

    // TODO: Prettify these ugly code
    fn to_postfix_vec(&self) -> Vec<String> {
        let mut sp_op = String::new();
        let mut num_char_string = String::new();
        let mut result: Vec<String> = vec![];
        let mut op_stack: Vec<String> = vec![];

        for c in self.chars() {
            if c.is_ascii_digit() || c == '.' {
                num_char_string.push(c);
            } else {
                if !num_char_string.is_empty() {
                    result.push(num_char_string.clone());
                    num_char_string.clear();
                }

                if c.is_ascii_alphabetic() {
                    sp_op.push(c);
                } else if c == '(' {
                    op_stack.push(c.to_string());
                    if !sp_op.is_empty() {
                        op_stack.push(sp_op.clone());
                        sp_op.clear();
                    }
                } else if c == ')' {
                    while let Some(op) = op_stack.pop() {
                        if op == '('.to_string() {
                            break;
                        }
                        result.push(op);
                    }
                } else {
                    let str_op = c.to_string();

                    while let Some(op) = op_stack.last() {
                        if priority(&str_op) > priority(op) {
                            break;
                        } else {
                            result.push(op_stack.pop().unwrap());
                        }
                    }
                    op_stack.push(str_op);
                }
            }
        }
        // Add remaining operands
        if !num_char_string.is_empty() {
            result.push(num_char_string.clone());
            num_char_string.clear();
        }
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
}

fn priority(s: &String) -> u8 {
    if s == "+" || s == "-" {
        1
    } else if s == "*" || s == "/" {
        2
    } else if s == "^" {
        3
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::ExprParser;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn infix_to_postfix_a() {
        let input = "1+20+33".to_string();
        assert_eq!(input.to_postfix(Some(" ")), "1 20 + 33 +".to_string());
    }

    #[test]
    fn infix_to_postfix_b() {
        let input = "8+cos(60)/2.5+6*abs(20-40)".to_string();
        assert_eq!(
            input.to_postfix(Some(" ")),
            "8 60 cos 2.5 / + 6 20 40 - abs * +".to_string()
        );
    }
}
