use std::ops::Add;
use std::num::Wrapping;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Fixed(Wrapping<i32>);

impl Fixed {
    pub fn new(v: i32) -> Fixed {
        Fixed(Wrapping(v << 16))
    }

    pub fn as_i32(self) -> i32 {
        let Fixed(Wrapping(v)) = self;
        v >> 16
    }
}

impl Add for Fixed {
    type Output = Fixed;

    fn add(self, Fixed(rhs): Fixed) -> Fixed {
        let Fixed(s) = self;
        Fixed(s + rhs)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Vec2<T: Add + Copy>(pub T, pub T);


impl<T> Add for Vec2<T> where T: Add<Output = T> + Copy {
    type Output = Vec2<T>;

    fn add(self, Vec2(x2, y2): Vec2<T>) -> Vec2<T> {
        let Vec2(x, y) = self;
        Vec2(x + x2, y + y2)
    }
}

pub type FixedVec = Vec2<Fixed>;

#[cfg(test)]
mod tests {
    use super::{Fixed, Vec2};

    #[test]
    fn add_fixed() {
        assert_eq!(Fixed::new(10) + Fixed::new(20), Fixed::new(30));
        assert_eq!((Fixed::new(10) + Fixed::new(-10)).as_i32(), 0);
    }

    #[test]
    fn add_vec() {
        assert_eq!(Vec2(Fixed::new(1), Fixed::new(2)) +
            Vec2(Fixed::new(2), Fixed::new(1)),
            Vec2(Fixed::new(3), Fixed::new(3)));
    }
}
