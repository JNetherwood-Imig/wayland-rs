pub struct Fixed(i32);

impl From<f32> for Fixed {
    fn from(value: f32) -> Self {
        Fixed((value * 256.0).round() as i32)
    }
}

impl From<Fixed> for f32 {
    fn from(value: Fixed) -> Self {
        value.0 as f32 / 256.0
    }
}

impl From<f64> for Fixed {
    fn from(value: f64) -> Self {
        Fixed((value * 256.0).round() as i32)
    }
}

impl From<Fixed> for f64 {
    fn from(value: Fixed) -> Self {
        value.0 as f64 / 256.0
    }
}

impl From<i32> for Fixed {
    fn from(value: i32) -> Self {
        Fixed(value >> 8)
    }
}

impl From<Fixed> for i32 {
    fn from(value: Fixed) -> Self {
        value.0 << 8
    }
}

impl From<u32> for Fixed {
    fn from(value: u32) -> Self {
        Fixed(value as i32 >> 8)
    }
}

impl From<Fixed> for u32 {
    fn from(value: Fixed) -> Self {
        value.0.max(0) as u32
    }
}
