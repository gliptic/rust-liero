use fixed::{FixedVec};
use list::{List, ItemAdder, ListIterator};
use nobject::{NObject, NObjectType};
use std::cell::UnsafeCell;
//use std::mem;
use worm::Worm;

pub struct Common {
    pub dummy_nobject_type: NObjectType,
}

#[repr(C)]
pub struct Sim<'common> {
    object_list: UnsafeCell<List<NObject<'common>>>,
    cur_object_index: u32,

    worm_list: List<Worm>,

    common: &'common Common,

    pub current_time: u32,

}

impl<'common> Sim<'common> {
    pub fn new(common: &Common) -> Sim {
        Sim {
            object_list: UnsafeCell::new(List::new(100)),
            cur_object_index: 0,

            worm_list: List::new(16),
            common: common,

            current_time: 0
        }
    }

    pub fn split_objects_iter_mut(&'common mut self) -> (&mut SimMinusObjects, ListIterator<NObject>) {
        unsafe {
            let it = (*self.object_list.get()).iter_mut();
            (self, it)
        }
    }
}

pub trait CommonSim {
    fn new_nobject(&mut self, pos: FixedVec, vel: FixedVec);
}

pub trait SimWorms {
    fn worm_iter(&mut self) -> ListIterator<Worm>;
}

pub trait SimMinusObjects: CommonSim + SimWorms {}

impl<'common> CommonSim for Sim<'common> {
    fn new_nobject(&mut self, pos: FixedVec, vel: FixedVec) {
        let v = NObject {
            ty: &self.common.dummy_nobject_type,
            pos: pos,
            vel: vel,
            time_to_die: self.current_time + 10,
            cur_frame: 0,
            index: self.cur_object_index
        };
        self.cur_object_index += 1;
        unsafe { (*self.object_list.get()).add(v); }
    }
}

impl<'common> SimWorms for Sim<'common> {
    fn worm_iter(&mut self) -> ListIterator<Worm> {
        self.worm_list.iter_mut()
    }
}

impl<'common> SimMinusObjects for Sim<'common> {}

#[cfg(test)]
mod tests {
    use super::{Sim, Common, SimMinusObjects};
    use nobject::{NObject, NObjectType};
    use list::{ItemAdder, ListIterator, InteractIterator};
    use fixed::Fixed;
    use std::num::Zero;

    fn iter<'a>(state: &mut SimMinusObjects, mut x: ListIterator<'a, NObject>) {
        while let Some(mut e) = x.next() {
            NObject::update(e, state);
        }
    }

    fn test<'a>(sim: &'a mut Sim<'a>) {
        let (mut sim_minus_object_list, mut x) = sim.split_objects_iter_mut();

        sim_minus_object_list.new_nobject(Zero::zero(), Zero::zero());

        iter(sim_minus_object_list, x);
    }

    #[test]
    fn parallel_access() {
        let common = Common {
            dummy_nobject_type: NObjectType { gravity: Fixed::new(0) }
        };

        let mut sim = Sim::new(&common);
        
        test(&mut sim);

        //assert_eq!(sim.current_time, 1);
    }
}