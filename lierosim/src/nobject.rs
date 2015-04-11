use fixed::{FixedVec};
use list;

pub struct NObject {
    pub pos: FixedVec,
    pub vel: FixedVec,

    // (Mostly) static values
    pub time_to_die: u32,
    pub cur_frame: u32,
    pub index: u32
}

#[cfg(test)]
mod tests {
    use super::NObject;
    use fixed::{Fixed, Vec2};
    use list;
    use list::InteractIterator;

    #[test]
    fn create_nobject() {
        let o = NObject {
            pos: Vec2(Fixed::new(0), Fixed::new(0)),
            vel: Vec2(Fixed::new(1), Fixed::new(1)),
            time_to_die: 0,
            cur_frame: 0,
            index: 0
        };

        assert_eq!(o.pos.0, Fixed::new(0));
        assert_eq!(o.vel.0, Fixed::new(1));
        assert_eq!(o.time_to_die, 0);
        assert_eq!(o.cur_frame, 0);
    }

    #[test]
    fn nobject_list() {
        let mut list: list::List<u32> = list::List::new(100);

        let mut it = list.iter_mut();

        while let Some(mut m) = it.next() {
            {
                let mut v = m.value();
                *v = 10;
            }

            for k in m.others_iter() {

            }

            m.remove();
        }
    }
}