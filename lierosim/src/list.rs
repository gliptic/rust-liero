use std::marker::PhantomData;
use std::mem;
use alloc::heap;

pub struct List<T: Copy> {
    beg: *mut T,
    end: *mut T,
    lim: *mut T,
}

impl<T: Copy> List<T> {
    pub fn new(max_count: u32) -> List<T> {
        unsafe {
            let m: *mut T = mem::transmute(
                heap::allocate(
                    mem::size_of::<T>().checked_mul(max_count as usize).unwrap(),
                    mem::min_align_of::<T>()));
            List {
                beg: m,
                end: m,
                lim: m.offset(max_count as isize)
            }
        }
    }

    pub fn iter_mut(&mut self) -> ListIterator<T> {
        ListIterator {
            list: self,
            cur: self.beg,
            phantom: PhantomData
        }
    }
}

pub struct ListIterator<'a, T: 'a + Copy> {
    list: &'a mut List<T>,
    cur: *mut T,
    phantom: PhantomData<&'a T>
}

pub struct SubIterator<'a, T: 'a> {
    exclude: *mut T,
    cur: *mut T,
    end: *mut T,
    phantom: PhantomData<&'a T>
}

impl<'a, T> Iterator for SubIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.cur != self.end {
            None
        } else {
            let mut p = self.cur;

            unsafe {
                if p == self.exclude {
                    p = p.offset(1);
                }
                self.cur = p.offset(1);
                Some(mem::transmute(p))
            }
        }
    }
}

pub struct ListItemRef<'a, T: 'a + Copy> {
    p: &'a mut T,
    list: &'a mut List<T>
}

impl<'a, T: 'a + Copy> ListItemRef<'a, T> {
    pub fn value(&mut self) -> &mut T {
        self.p
    }

    pub fn remove(self) {
        // Swap with last
        unsafe {
            *self.p = *self.list.end;
            self.list.end = self.list.end.offset(-1);
        }
    }

    pub fn others_iter<'b>(&'b mut self) -> SubIterator<'b, T> {
        SubIterator {
            exclude: self.p as *mut T,
            cur: self.list.beg,
            end: self.list.end,
            phantom: PhantomData
        }
    }
}

pub trait InteractIterator {
    type Item;

    fn next<'a>(&'a mut self) -> Option<ListItemRef<'a, <Self as InteractIterator>::Item>>;
}

impl<'a, T: 'a + Copy> InteractIterator for ListIterator<'a, T> {
    type Item = T;

    fn next<'b>(&'b mut self) -> Option<ListItemRef<'b, T>> {
        if self.cur != self.list.end {
            let p = self.cur;

            unsafe {
                self.cur = p.offset(1);
                Some(ListItemRef {
                    p: &mut *(p as *mut T),
                    list: self.list
                })
            }
        } else {
            None
        }
    }
}