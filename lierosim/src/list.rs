use std::marker::PhantomData;
use std::mem;
use alloc::heap;

pub struct List<T: Copy> {
    beg: *mut T,
    end: *mut T,
    lim: *mut T,
}

pub trait ItemAdder<T: Copy> {
    fn add(&mut self, item: T);
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

impl<T: Copy> ItemAdder<T> for List<T> {
    fn add(&mut self, item: T) {
        if self.end != self.lim {
            unsafe {
                *self.end = item;
                self.end = self.end.offset(1);
            }
        }
    }
}

impl<T: Copy> Drop for List<T> {
    fn drop(&mut self) {
        unsafe { heap::deallocate(mem::transmute(self.beg), (self.lim as usize) - (self.beg as usize), mem::min_align_of::<T>()); }
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
        let mut p = self.cur;

        if p == self.exclude {
            unsafe { p = p.offset(1) }
        }

        if p == self.end {
            None
        } else {
            unsafe {
                self.cur = p.offset(1);
                Some(&mut *p)
            }
        }
    }
}

pub struct ListItemRef<'a, T: 'a + Copy> {
    cur: &'a mut *mut T,
    list: &'a mut List<T>
}

impl<'a, T: 'a + Copy> ItemAdder<T> for ListItemRef<'a, T> {
    fn add(&mut self, item: T) {
        self.list.add(item)
    }
}

impl<'a, T: 'a + Copy> ItemAdder<T> for ListIterator<'a, T> {
    fn add(&mut self, item: T) {
        self.list.add(item)
    }
}

impl<'a, T: 'a + Copy> ListItemRef<'a, T> {
    pub fn value(&mut self) -> &mut T {
        unsafe { &mut **self.cur }
    }

    pub fn remove(self) {
        // Swap with last
        unsafe {
            self.list.end = self.list.end.offset(-1);
            **self.cur = *self.list.end;
            mem::forget(self);
        }
    }

    pub fn others_iter<'b>(&'b mut self) -> SubIterator<'b, T> {
        SubIterator {
            exclude: *self.cur as *mut T,
            cur: self.list.beg,
            end: self.list.end,
            phantom: PhantomData
        }
    }
}

impl<'a, T: 'a + Copy> Drop for ListItemRef<'a, T> {
    fn drop(&mut self) {
        unsafe { *self.cur = self.cur.offset(1); }
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
            Some(ListItemRef {
                cur: &mut self.cur,
                list: self.list
            })
        } else {
            None
        }
    }
}