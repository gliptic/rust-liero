use fixed::{FixedVec};
use list::ListItemRef;

#[derive(Copy, Clone)]
pub struct NObject {
    pub pos: FixedVec,
    pub vel: FixedVec,

    // (Mostly) static values
    pub time_to_die: u32,
    pub cur_frame: u32,
    pub index: u32
}

impl NObject {
    fn update<'a>(mut obj: ListItemRef<'a, NObject>) {
        let o = obj.value();
        o.pos = o.pos + o.vel;
    }
}

#[cfg(test)]
mod tests {
    use super::NObject;
    use fixed::{Fixed, Vec2};
    use list;
    use list::{InteractIterator, ItemAdder};

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

        {
            let mut it = list.iter_mut();

            it.add(1);
            it.add(2);
            it.add(3);

            // This will be processed like this:
            // 1 2 3 ->
            // 2 2 3 ->
            // 2 4 5 ->

            // 5 4 ->
            // 6 4 ->
            // 6 6 ->

            // 6 ->
            // 7

            while let Some(mut m) = it.next() {
                {
                    let v = m.value();
                    *v = *v + 1;
                }

                for k in m.others_iter() {
                    *k = *k + 2;
                }

                if *m.value() & 1 == 0 {
                    m.remove();
                }
            }
        }

        {
            let mut it = list.iter_mut();

            let mut count = 0;

            while let Some(mut m) = it.next() {
                assert_eq!(*m.value(), 7);
                count += 1;
            }

            assert_eq!(count, 1);
        }
    }
}