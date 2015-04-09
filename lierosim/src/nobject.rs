use fixed::{FixedVec};

pub struct NObject {
    pub pos: FixedVec,
    pub vel: FixedVec,

    // Static values
    pub time_to_die: u32,
    pub cur_frame: u32,
    pub index: u32
}

#[cfg(test)]
mod test {
    use super::NObject;
    use fixed::{Fixed, Vec2};

    #[test]
    fn create_nobject() {
        let o = NObject {
            pos: Vec2(Fixed::new(0), Fixed::new(0)),
            vel: Vec2(Fixed::new(1), Fixed::new(1)),
            time_to_die: 0,
            cur_frame: 0
        };

        assert_eq!(o.pos.0, Fixed::new(0));
        assert_eq!(o.vel.0, Fixed::new(1));
        assert_eq!(o.time_to_die, 0);
        assert_eq!(o.cur_frame, 0);
    }
}