
/// A enum type that represent both regular integer (i32) and float (f64)
/// 
/// This enum currently has two varient, `Number::Integer(i32)` and `Number::Float(f64)`.
pub enum Number {
    Integer(i32),
    Float(f64),
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self::Integer(n)
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Self::Float(n)
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Integer(val) => val.fmt(f),
            &Self::Float(val) => val.fmt(f),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Integer(val) => val.fmt(f),
            &Self::Float(val) => val.fmt(f),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match self {
            &Self::Integer(val) => match other {
                &Self::Integer(oth_val) => val == oth_val,
                &Self::Float(oth_val) => (f64::from(val) - oth_val).abs() < f64::EPSILON * 100.0
            },
            &Self::Float(val) => match other {
                &Self::Integer(oth_val) => (val - f64::from(oth_val)).abs() < f64::EPSILON * 100.0,
                &Self::Float(oth_val) => (val - oth_val).abs() < f64::EPSILON * 100.0
            },
        }
    }
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Integer(val) => match rhs {
                Self::Integer(rhs_val) => Self::Integer(val + rhs_val),
                Self::Float(rhs_val) => Self::Float(f64::from(val) + rhs_val),
            },
            Self::Float(val) => match rhs {
                Self::Integer(rhs_val) => Self::Float(val + f64::from(rhs_val)),
                Self::Float(rhs_val) => Self::Float(val + rhs_val),
            },
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Integer(val) => match rhs {
                Self::Integer(rhs_val) => Self::Integer(val - rhs_val),
                Self::Float(rhs_val) => Self::Float(f64::from(val) - rhs_val),
            },
            Self::Float(val) => match rhs {
                Self::Integer(rhs_val) => Self::Float(val - f64::from(rhs_val)),
                Self::Float(rhs_val) => Self::Float(val - rhs_val),
            },
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Integer(val) => match rhs {
                Self::Integer(rhs_val) => Self::Integer(val * rhs_val),
                Self::Float(rhs_val) => Self::Float(f64::from(val) * rhs_val),
            },
            Self::Float(val) => match rhs {
                Self::Integer(rhs_val) => Self::Float(val * f64::from(rhs_val)),
                Self::Float(rhs_val) => Self::Float(val * rhs_val),
            },
        }
    }
}

impl std::ops::Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Integer(val) => match rhs {
                Self::Integer(rhs_val) => {
                    if rhs_val == 0 {
                        panic!("Cannot divide by zero.")
                    }
                    Self::Float(f64::from(val) / f64::from(rhs_val))
                }
                Self::Float(rhs_val) => {
                    if rhs_val == 0.0 {
                        panic!("Cannot divide by zero.")
                    }
                    Self::Float(f64::from(val) / rhs_val)
                }
            },
            Self::Float(val) => match rhs {
                Self::Integer(rhs_val) => {
                    if rhs_val == 0 {
                        panic!("Cannot divide by zero.")
                    }
                    Self::Float(val / f64::from(rhs_val))
                }
                Self::Float(rhs_val) => {
                    if rhs_val == 0.0 {
                        panic!("Cannot divide by zero.")
                    }
                    Self::Float(val / rhs_val)
                }
            },
        }
    }
}
