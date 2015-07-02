use fixed::{Vec2, F64Vec};
use list::{ListItemRef, Bruteforce, BroadphaseNode};
use level::{Level, Mat};
use sim::SimMinusObjects;
use std::num::Zero;

#[derive(Copy, Clone)]
pub struct NObjectType {
    pub gravity: f64,
    pub splinter_count: u32,
    pub splinter_type: u32,
    pub time_to_live: u32,
    pub time_to_live_v: u32,
}

#[derive(Copy, Clone)]
pub struct NObject {
    //pub ty: &'a NObjectType,
    pub pos: F64Vec,
    pub vel: F64Vec,

    // (Mostly) static values
    pub ty_idx: u32,
    pub time_to_die: u32,
    pub cur_frame: i32,
    pub index: u32,
    pub cell_index: u32
}

#[derive(Eq, PartialEq)]
enum ObjState {
    Alive,
    Removed,
    Exploded
}

impl NObject {
    fn explode_obj(
        ty: &NObjectType,
        pos: F64Vec,
        vel: F64Vec,
        state: &mut SimMinusObjects) {

        // TODO: Schedule sobjects
        // TODO: Play sound
        for _ in 0..ty.splinter_count {
            // TODO: Create splinters
            state.new_nobject(0, pos, vel); // Testing
        }
        // TODO: Create dirt effect
    }

    pub fn update(mut obj: ListItemRef<NObject, Bruteforce>, state: &mut SimMinusObjects) {
        let mut obj_state = ObjState::Alive;
        let mut iteration = 0;
        let (mut pos, mut vel);
        let ty;

        {
            let o = obj.value();
            ty = state.get_nobject_type(o.ty_idx);
            pos = o.pos;
            vel = o.vel;
        }

        let max_iteration = 1; // param

        'repeat: while iteration < max_iteration {
            iteration = iteration + 1;

            pos = pos + vel;

            let inewpos = (pos + vel).as_i32();
            let ipos = pos.as_i32();

            let mut bounced = false;

            let level = Level::new();
            let current_time = 0; // This is called cycles in original

            // This should be set to Fixed::from_frac(w.bounce, 100)
            let bounce = 1.0;
            let expl_ground = false;
            // This should be set to Fixed::new(1) if this is a wobject and w.bounce == 100, otherwise Fixed::from_frac(4, 5)
            let friction = 1.0;
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
                if bounce == 0.0 {
                    vel = Zero::zero();
                    if expl_ground {
                        // TODO: Draw on map for nobject
                        obj_state = ObjState::Exploded;
                        break 'repeat;
                    }
                }
                animate = false; // TODO: Nobjects are animated in this case too
            } else {
                if !bounced {
                    // TODO: Sobject trail for nobject
                }
                vel.1 = vel.1 + ty.gravity;
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
                    obj_state = ObjState::Exploded;
                    break 'repeat;
                }
            }

            // TODO: Coldet with worms

        }

        if obj_state == ObjState::Alive {
            {
                // Update pos/vel
                let o = obj.value();
                o.pos = pos;
                o.vel = vel;
            }
            obj.update_broadphase();
        } else {
            if obj_state == ObjState::Exploded {
                NObject::explode_obj(ty, pos, vel, state);
            }
            obj.remove();
        }
    }
}

impl BroadphaseNode for NObject {
    fn set_cell(&mut self, index: u32) {
        self.cell_index = index;
    }

    fn get_cell(&mut self) -> u32 {
        self.cell_index
    }

    fn pos(&self) -> Vec2<i32> {
        self.pos.as_i32()
    }
}

#[cfg(test)]
mod tests {
    use super::{NObject, NObjectType};
    use fixed::{Vec2};
    //use list::{InteractIterator, ItemAdder};

    #[test]
    fn create_nobject() {
        let o = NObject {
            ty_idx: 0,
            pos: Vec2(0.0, 0.0),
            vel: Vec2(1.0, 1.0),
            time_to_die: 0,
            cur_frame: 0,
            index: 0,
            cell_index: 0
        };

        assert_eq!(o.pos.0, 0.0);
        assert_eq!(o.vel.0, 1.0);
        assert_eq!(o.time_to_die, 0);
        assert_eq!(o.cur_frame, 0);
    }

/*
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
    }*/
}