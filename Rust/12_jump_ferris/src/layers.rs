#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Layer {
    Background = 1,
    BackgroundDeco,
    Platforms,
    Characters,
    UI = 998,
    Other = 999,
}

impl Into<f32> for Layer {
    fn into(self) -> f32 {
        // enum has to be cast to integer first
        self as i32 as f32
    }
}

impl Layer {
    pub fn to_f32(&self) -> f32 {
        (*self).into()
    }
}

#[cfg(test)]
mod layer_tests {
    use super::Layer::*;
    #[test]
    fn layer_to_f32() {
        assert_eq!(Background.to_f32(), 1.0);
        assert_eq!(BackgroundDeco.to_f32(), 2.0);
        assert_eq!(Platforms.to_f32(), 3.0);
        assert_eq!(Other.to_f32(), 999.0);
    }
}
