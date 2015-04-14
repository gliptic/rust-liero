use fixed::{Fixed, Vec2, FixedVec};
use list::ListItemRef;
use sim::SimMinusObjects;
use std::num::Zero;

pub struct NObjectType {
    pub gravity: Fixed
}

#[derive(Copy, Clone)]
pub struct NObject<'a> {
    pub ty: &'a NObjectType,
    pub pos: FixedVec,
    pub vel: FixedVec,

    // (Mostly) static values
    pub time_to_die: u32,
    pub cur_frame: i32,
    pub index: u32
}

// Level mock
struct Level;
struct Mat;
impl Level {
    fn mat(&self, _: Vec2<i32>) -> Mat {
        Mat
    }
}
impl Mat {
    fn dirt_rock(&self) -> bool {
        true
    }
}

impl<'s> NObject<'s> {
    fn explode_obj(obj: &ListItemRef<NObject>, pos: FixedVec, vel: FixedVec, state: &mut SimMinusObjects) {

        // TODO: Schedule sobjects
        // TODO: Play sound
        // TODO: Create splinters
        // TODO: Create dirt effect
        state.new_nobject(pos, vel); // Testing
    }

    pub fn update<'a>(mut obj: ListItemRef<'a, NObject>, state: &mut SimMinusObjects) {
        let (mut do_explode, mut do_remove) = (false, false);
        let mut iteration = 0;

        let max_iteration = 1; // param

        loop {
            iteration = iteration + 1;

            let (pos, mut vel);

            {
                let o = obj.value();
                pos = o.pos;
                vel = o.vel;
                o.pos = pos + vel;
            }

            let inewpos = (pos + vel).as_i32();
            let ipos = pos.as_i32();

            let mut bounced = false;

            let level = Level;
            let current_time = 0; // This is called cycles in original

            // This should be set to Fixed::from_frac(w.bounce, 100)
            let bounce = Fixed::new(1);
            let expl_ground = false;
            // This should be set to Fixed::new(1) if this is a wobject and w.bounce == 100, otherwise Fixed::from_frac(4, 5)
            let friction = Fixed::new(1);
            let gravity = Fixed::new(1);
            let num_frames = 0;
            let directional_animation = true; // This is called loopAnim in original

            {
                

                if level.mat(Vec2(inewpos.0, ipos.1)).dirt_rock() {
                    vel = Vec2(-vel.0 * bounce, vel.1 * friction);
                    bounced = true;
                }

                if level.mat(Vec2(ipos.0, inewpos.1)).dirt_rock() {
                    vel = Vec2(vel.0 * friction, -vel.1 * bounce);
                    bounced = true;
                }
            }

            // TODO: Blood trail for nobject

            // TODO: Speed scaling for wobject
            // TODO: Sobject trail for wobject
            // TODO: Nobject trail for wobject


            // TODO: Object collisions for wobject

            // vel may have changed. Adjust and shadow old variable.
            let inewpos = (pos + vel).as_i32();

            // TODO: Limit o.pos if inewpos is outside the level

            let animate;

            if level.mat(inewpos).dirt_rock() {
                if bounce == Fixed::new(0) {
                    vel = Zero::zero();
                    if expl_ground {
                        // TODO: Draw on map for nobject
                        do_explode = true;
                    }
                }
                animate = false; // TODO: Nobjects are animated in this case too
            } else {
                if !bounced {
                    // TODO: Sobject trail for nobject
                }
                vel.1 = vel.1 + gravity;
                animate = true;
            }

            {
                let o = obj.value();

                if num_frames > 0 && animate {

                    if (current_time & 7) == 0 {
                        if !directional_animation || o.vel.0 < Zero::zero() {
                            o.cur_frame = o.cur_frame - 1;
                            if o.cur_frame < 0 {
                                o.cur_frame = num_frames;
                            }
                        } else if o.vel.0 > Zero::zero() {
                            o.cur_frame = o.cur_frame + 1;
                            if o.cur_frame > num_frames {
                                o.cur_frame = 0;
                            }
                        }
                    }
                }

                if current_time > o.time_to_die {
                    do_explode = true;
                }
            }

            // TODO: Coldet with worms

            if do_explode {
                NObject::explode_obj(&obj, pos, vel, state);
                break;
            } else if do_remove {
                break;
            }

            {
                // Update velocity
                obj.value().vel = vel;
            }

            if iteration >= max_iteration {
                break;
            }
        }

        if do_explode || do_remove {
            obj.remove();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NObject, NObjectType};
    use fixed::{Fixed, Vec2};
    use list;
    use list::{InteractIterator, ItemAdder};

    #[test]
    fn create_nobject() {
        let t = NObjectType { gravity: Fixed::new(0) };

        let o = NObject {
            ty: &t,
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
            list.add(1);
            list.add(2);
            list.add(3);

            let mut it = list.iter_mut();

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