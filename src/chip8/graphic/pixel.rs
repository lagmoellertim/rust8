use std::ops::BitXor;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Pixel {
    On,
    #[default]
    Off,
}

impl BitXor for Pixel {
    type Output = Pixel;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Pixel::On, Pixel::Off) => Pixel::On,
            (Pixel::Off, Pixel::On) => Pixel::On,
            _ => Pixel::Off,
        }
    }
}

impl From<bool> for Pixel {
    fn from(value: bool) -> Self {
        if value {
            Pixel::On
        } else {
            Pixel::Off
        }
    }
}
