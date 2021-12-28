#![allow(dead_code)]

use crate::number::Number;

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

pub fn mul(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() || b.is_none() {
        panic!("Fail to multiply numbers. (Multiplication requires two numbers)");
    }

    a.unwrap() * b.unwrap()
}

pub fn div(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() || b.is_none() {
        panic!("Fail to divide numbers. (Division requires two numbers)");
    }

    a.unwrap() / b.unwrap()
}

pub fn pow(a: Option<Number>, b: Option<Number>) -> Number {
    if a.is_none() || b.is_none() {
        panic!("Fail to raise power of a number. (Pow requires two numbers)");
    }

    match a.unwrap() {
        Number::Integer(val) => match b.unwrap() {
            Number::Integer(p_val) => {
                if p_val > 0 {
                    Number::Integer(val.pow(p_val as u32))
                } else {
                    Number::Float(f64::from(val).powi(p_val))
                }
            }
            Number::Float(p_val) => Number::Float(f64::from(val).powf(p_val)),
        },
        Number::Float(val) => match b.unwrap() {
            Number::Integer(p_val) => Number::Float(val.powi(p_val)),
            Number::Float(p_val) => Number::Float(val.powf(p_val)),
        },
    }
}

pub fn sqrt(a: Option<Number>) -> Number {
    if a.is_none() {
        panic!("Fail to calculate square root of a number. (sqrt requires at lease one operand.)");
    }

    match a.unwrap() {
        Number::Integer(val) => Number::Float(f64::from(val).sqrt()),
        Number::Float(val) => Number::Float(val.sqrt()),
    }
}

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

fn trianglar_ops_common(
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

pub fn cos(deg: Option<Number>) -> Number {
    trianglar_ops_common(deg, true, "cosine", f64::cos, false)
}

pub fn sin(deg: Option<Number>) -> Number {
    trianglar_ops_common(deg, true, "sine", f64::sin, false)
}

pub fn tan(deg: Option<Number>) -> Number {
    trianglar_ops_common(deg, true, "tangent", f64::tan, false)
}

pub fn acos(val: Option<Number>) -> Number {
    trianglar_ops_common(val, false, "arccosine", f64::acos, true)
}

pub fn asin(val: Option<Number>) -> Number {
    trianglar_ops_common(val, false, "arcsine", f64::asin, true)
}

pub fn atan(val: Option<Number>) -> Number {
    trianglar_ops_common(val, false, "arctangent", f64::atan, true)
}
