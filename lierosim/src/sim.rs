use fixed::{F64Vec};
use list::{List, ItemAdder, ListIterator, Bruteforce};
use nobject::{NObject, NObjectType};
use std::cell::UnsafeCell;
use common::Common;
use rand::Xorshift;
//use std::mem;
use worm::Worm;


#[repr(C)]
pub struct Sim<'common> {
    object_list: UnsafeCell<List<NObject, Bruteforce>>,
    cur_object_index: u32,

    worm_list: UnsafeCell<List<Worm, Bruteforce>>,

    common: &'common Common,

    pub current_time: u32,
    pub rand: Xorshift

}

impl<'common> Sim<'common> {
    pub fn new(common: &Common) -> Sim {
        Sim {
            object_list: UnsafeCell::new(List::new(100)),
            cur_object_index: 0,

            worm_list: UnsafeCell::new(List::new(16)),
            common: common,

            current_time: 0,
            rand: Xorshift::new()
        }
    }

    pub fn split_objects_iter_mut(&'common mut self) -> (&mut SimMinusObjects, ListIterator<NObject, Bruteforce>) {
        unsafe {
            let it = (*self.object_list.get()).iter_mut();
            (self, it)
        }
    }

    pub fn split_worm_iter_mut(&'common mut self) -> (&mut SimMinusObjects, ListIterator<Worm, Bruteforce>) {
        unsafe {
            let it = (*self.worm_list.get()).iter_mut();
            (self, it)
        }
    }
}

pub trait CommonSim<'common> {
    fn new_nobject(&mut self, ty_idx: u32, pos: F64Vec, vel: F64Vec);
    fn get_nobject_type(&self, ty_idx: u32) -> &'common NObjectType;
    fn rand(&mut self) -> &mut Xorshift;
}

pub trait SimWorms {
    fn worm_iter(&mut self) -> ListIterator<Worm, Bruteforce>;
}

pub trait SimObjects<'common> {
    fn objects_iter(&'common mut self) -> ListIterator<NObject, Bruteforce>;
}

impl<'common> CommonSim<'common> for Sim<'common> {
    fn new_nobject(&mut self, ty_idx: u32, pos: F64Vec, vel: F64Vec) {
        let ty = &self.common.nobject_types[ty_idx as usize];

        let v = NObject {
            ty_idx: ty_idx,
            pos: pos,
            vel: vel,
            time_to_die: self.current_time + ty.time_to_live - self.rand.next_max_u32(ty.time_to_live_v),
            cur_frame: 0,
            index: self.cur_object_index,
            cell_index: 0
        };
        self.cur_object_index += 1;
        unsafe { (*self.object_list.get()).add(v); }
    }

    fn get_nobject_type(&self, ty_idx: u32) -> &'common NObjectType {
        &self.common.nobject_types[ty_idx as usize]
    }

    fn rand(&mut self) -> &mut Xorshift {
        &mut self.rand
    }
}

impl<'common> SimWorms for Sim<'common> {
    fn worm_iter(&mut self) -> ListIterator<Worm, Bruteforce> {
        unsafe { (*self.worm_list.get()).iter_mut() }
    }
}

impl<'common> SimObjects<'common> for Sim<'common> {
    fn objects_iter(&'common mut self) -> ListIterator<NObject, Bruteforce> {
        unsafe { (*self.object_list.get()).iter_mut() }
    }
}

pub trait SimMinusObjects<'common>: CommonSim<'common> + SimWorms {}
pub trait SimMinusWorms<'common>: CommonSim<'common> + SimObjects<'common> {}

impl<'common> SimMinusObjects<'common> for Sim<'common> {}
impl<'common> SimMinusWorms<'common> for Sim<'common> {}

#[cfg(test)]
mod tests {
    use fixed::Fixed;
    use list::{ListIterator, InteractIterator, Bruteforce};
    use nobject::{NObject, NObjectType};
    use std::num::Zero;
    use common::Common;
    use super::{Sim, SimMinusObjects};

    fn iter(state: &mut SimMinusObjects, mut x: ListIterator<NObject, Bruteforce>) {
        while let Some(e) = x.next() {
            NObject::update(e, state);
        }
    }

    fn test<'a>(sim: &'a mut Sim<'a>) {
        let (mut sim_minus_object_list, x) = sim.split_objects_iter_mut();

        sim_minus_object_list.new_nobject(0, Zero::zero(), Zero::zero());

        iter(sim_minus_object_list, x);
    }

    #[test]
    fn parallel_access() {
        let common = Common::new();

        let mut sim = Sim::new(&common);
        
        test(&mut sim);

        //assert_eq!(sim.current_time, 1);
    }
}