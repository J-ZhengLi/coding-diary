#[allow(dead_code)]
pub mod random {
    use rand::{thread_rng, Rng};
    use rand::seq::SliceRandom;

    // generate a float number between 0 and 1
    pub fn float() -> f64 {
        thread_rng().gen_range(0.0..1.0)
    }

    // generate a float number within specific range
    pub fn float_in_range(min: f64, max: f64) -> f64 {
        if min > max {
            eprintln!(
                "Invalid argument at Random.float_in_range(min, max). 
                min value cannot be greater than max value. ");
            return 0.0;
        }
        thread_rng().gen_range(min..max)
    }

    // generate an integer bounded by the size of a i32
    pub fn integer() -> i32 {
        thread_rng().gen::<i32>()
    }

    // generate an interger with specific range
    pub fn integer_in_range(min: i32, max: i32) -> i32 {
        if min > max {
            eprintln!(
                "Invalid argument at Random.integer_in_range(min, max). 
                min value cannot be greater than max value. ");
            return 0;
        }
        thread_rng().gen_range(min..max)
    }

    pub fn element_in_slice<T>(list: &[T]) -> Option<&T> {
        list.choose(&mut thread_rng())
    }
}