#![allow(dead_code)]

use crate::number::Number;

/// Adding `Number`s
/// 
/// Note: If one of the argument is `None`, it will be treated as 0.
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::add, number::Number};
/// 
/// let a = Number::from(5);
/// let b = Number::from(2.0);
/// assert_eq!(add(Some(a), Some(b)), Number::Float(7.0))
/// ```
pub fn add(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() && b.is_none() {
        panic!("Fail to add numbers. (No number is given)");
    }

    let left = if a.is_none() {
        Number::Integer(0)
    } else {
        a.unwrap()
    };
    let right = if b.is_none() {
        Number::Integer(0)
    } else {
        b.unwrap()
    };

    left + right
}

/// Substract `Number`s
/// 
/// Note: If one of the argument is `None`, it will be treated as 0.
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::sub, number::Number};
/// 
/// let a = Number::from(10);
/// let b = Number::from(5);
/// assert_eq!(sub(Some(a), Some(b)), Number::Integer(5))
/// ```
pub fn sub(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() && b.is_none() {
        panic!("Fail to subtract numbers. (No number is given)");
    }

    let left = if a.is_none() {
        Number::Integer(0)
    } else {
        a.unwrap()
    };
    let right = if b.is_none() {
        Number::Integer(0)
    } else {
        b.unwrap()
    };

    left - right
}

/// Multiply `Number`s
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::mul, number::Number};
/// 
/// let a = Number::from(10);
/// let b = Number::from(5.0);
/// assert_eq!(mul(Some(a), Some(b)), Number::Float(50.0))
/// ```
pub fn mul(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() || b.is_none() {
        panic!("Fail to multiply numbers. (Multiplication requires two numbers)");
    }

    a.unwrap() * b.unwrap()
}

/// Divide `Number`s
/// 
/// Note: Program will panic if the second input has value of 0.
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::div, number::Number};
/// 
/// let a = Number::from(81);
/// let b = Number::from(9);
/// assert_eq!(div(Some(a), Some(b)), Number::Integer(9))
/// ```
pub fn div(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() || b.is_none() {
        panic!("Fail to divide numbers. (Division requires two numbers)");
    }

    a.unwrap() / b.unwrap()
}

/// Raise power of a `Number` by another `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::pow, number::Number};
/// 
/// let n = Number::from(10);
/// let p = Number::from(3);
/// assert_eq!(pow(Some(n), Some(p)), Number::Integer(1000))
/// ```
pub fn pow(n: Option<Number>, p: Option<Number>) -> Number {
    if n.is_none() || p.is_none() {
        panic!("Fail to raise power of a number. (Pow requires two numbers)");
    }

    match n.unwrap() {
        Number::Integer(val) => match p.unwrap() {
            Number::Integer(p_val) => {
                if p_val > 0 {
                    Number::Integer(val.pow(p_val as u32))
                } else {
                    Number::Float(f64::from(val).powi(p_val))
                }
            }
            Number::Float(p_val) => Number::Float(f64::from(val).powf(p_val)),
        },
        Number::Float(val) => match p.unwrap() {
            Number::Integer(p_val) => Number::Float(val.powi(p_val)),
            Number::Float(p_val) => Number::Float(val.powf(p_val)),
        },
    }
}

/// Calculate square root of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::sqrt, number::Number};
/// 
/// let a = Number::from(64);
/// assert_eq!(sqrt(Some(a)), Number::Float(8.0))
/// ```
pub fn sqrt(n: Option<Number>) -> Number {
    if n.is_none() {
        panic!("Fail to calculate square root of a number. (sqrt requires at lease one operand.)");
    }

    match n.unwrap() {
        Number::Integer(val) => Number::Float(f64::from(val).sqrt()),
        Number::Float(val) => Number::Float(val.sqrt()),
    }
}

/// Get the absolute value of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::abs, number::Number};
/// 
/// let a = Number::from(-1);
/// assert_eq!(abs(Some(a)), Number::Integer(1))
/// ```
pub fn abs(a: Option<Number>) -> Number {
    if a.is_none() {
        panic!(
            "Fail to calculate the absolute value of a number. \
            (abs requires at lease one operand.)"
        );
    }

    match a.unwrap() {
        Number::Integer(val) => Number::Integer(val.abs()),
        Number::Float(val) => Number::Float(val.abs()),
    }
}

fn deg2rad<T>(deg_val: T) -> f64
where
    f64: From<T>,
{
    f64::from(deg_val) * std::f64::consts::PI / 180.0
}

fn rad2deg<T>(rad_val: T) -> f64
where
    f64: From<T>,
{
    f64::from(rad_val) * 180.0 / std::f64::consts::PI
}

fn trig_ops_common(
    val: Option<Number>,
    use_deg: bool,
    fn_name: &str,
    ops_fn: fn(f64) -> f64,
    res_use_deg: bool,
) -> Number {
    if val.is_none() {
        panic!(
            "Fail to compute \'{}\' of a number. (No number was given)",
            fn_name
        );
    }

    match val.unwrap() {
        Number::Integer(val) => {
            let rad: f64 = if use_deg {
                deg2rad(val)
            } else {
                f64::from(val)
            };
            let res: f64 = if res_use_deg {
                rad2deg(ops_fn(rad))
            } else {
                ops_fn(rad)
            };
            Number::Float(res)
        }
        Number::Float(val) => {
            let rad: f64 = if use_deg { deg2rad(val) } else { val };
            let res: f64 = if res_use_deg {
                rad2deg(ops_fn(rad))
            } else {
                ops_fn(rad)
            };
            Number::Float(res)
        }
    }
}

/// Calculate cosine of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::cos, number::Number};
/// 
/// let n: Number = Number::from(180.0);
/// let m: Number = Number::from(0);
/// assert_eq!(cos(Some(n)), Number::Float(-1.0));
/// assert_eq!(cos(Some(m)), Number::Float(1.0));
/// ```
pub fn cos(deg: Option<Number>) -> Number {
    trig_ops_common(deg, true, "cosine", f64::cos, false)
}

/// Calculate sine of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::sin, number::Number};
/// 
/// let n: Number = Number::from(-30.0);
/// let m: Number = Number::from(0);
/// assert_eq!(sin(Some(n)), Number::Float(-0.5));
/// assert_eq!(sin(Some(m)), Number::Float(0.0));
/// ```
pub fn sin(deg: Option<Number>) -> Number {
    trig_ops_common(deg, true, "sine", f64::sin, false)
}

/// Calculate tangent of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::tan, number::Number};
/// 
/// let n: Number = Number::from(45.0);
/// let m: Number = Number::from(0);
/// assert_eq!(tan(Some(n)), Number::Float(1.0));
/// assert_eq!(tan(Some(m)), Number::Float(0.0));
/// ```
pub fn tan(deg: Option<Number>) -> Number {
    trig_ops_common(deg, true, "tangent", f64::tan, false)
}

/// Calculate arc cosine of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::acos, number::Number};
/// 
/// let n: Number = Number::from(0.5);
/// assert_eq!(acos(Some(n)), Number::Float(60.0));
/// ```
pub fn acos(val: Option<Number>) -> Number {
    trig_ops_common(val, false, "arccosine", f64::acos, true)
}

/// Calculate arc sine of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::asin, number::Number};
/// 
/// let n: Number = Number::from(0.5);
/// assert_eq!(asin(Some(n)), Number::Float(30.0));
/// ```
pub fn asin(val: Option<Number>) -> Number {
    trig_ops_common(val, false, "arcsine", f64::asin, true)
}

/// Calculate arc tangent of a `Number`
/// 
/// # Example
/// ```rust
/// use calculator_util::{operations::atan, number::Number};
/// 
/// let n: Number = Number::from(1.0);
/// assert_eq!(atan(Some(n)), Number::Float(45.0));
/// ```
pub fn atan(val: Option<Number>) -> Number {
    trig_ops_common(val, false, "arctangent", f64::atan, true)
}
