//use alloc::heap;
use std::marker::PhantomData;
use std::mem;
//use std::num::Zero;
use fixed::Vec2;

pub struct List<T: BroadphaseNode, B: Broadphase> {
    /*
    beg: *mut T,
    end: *mut T,
    lim: *mut T,
    */
    items: Vec<T>,
    broadphase: B,
}

pub trait BroadphaseNode: Copy {
    fn set_cell(&mut self, index: u32);
    fn get_cell(&mut self) -> u32;
    fn pos(&self) -> Vec2<i32>;
}

pub trait Broadphase {
    fn new(max_count: u32) -> Self;
    fn insert(&mut self, idx: u32, pos: Vec2<i32>) -> u32;
    fn update(&mut self, idx: u32, pos: Vec2<i32>, cur_index: u32) -> u32;
    fn swap_remove(&mut self, idx_: u32, last_idx_: u32);
    //fn remove(&mut self, idx: u32);
}


// Cell broadphase
#[derive(Copy, Clone)]
pub struct CellNode {
    next: u32,
    prev: u32,
}

const CELL_SHIFT: u32 = 4;
const WORLD_SHIFT: u32 = 9;
const GRID_SHIFT: u32 = WORLD_SHIFT - CELL_SHIFT;
const GRID_WIDTH: u32 = 1u32 << GRID_SHIFT;
const GRID_SIZE: u32 = GRID_WIDTH * GRID_WIDTH;
const CELL_MASK: u32 = GRID_WIDTH - 1;

pub struct Cellphase {
    cells: Vec<CellNode>
}

impl Broadphase for Cellphase {
    fn new(max_count: u32) -> Cellphase {
        let mut cells = Vec::with_capacity((GRID_SIZE + max_count) as usize);

        for i in 0..(GRID_WIDTH * GRID_WIDTH + max_count) {
            cells.push(CellNode {
                prev: i,
                next: i
            });
        }

        Cellphase {
            cells: cells
        }
    }

    fn insert(&mut self, idx_: u32, pos: Vec2<i32>) -> u32 {
        let (cx, cy) = (pos.0 as u32 >> CELL_SHIFT, (pos.1 as u32 >> CELL_SHIFT) << GRID_SHIFT);
        let new_cell = (cx + cy) & (CELL_MASK | (CELL_MASK << GRID_SHIFT));

        let idx = idx_ + GRID_SIZE;
        let sp = self.cells[new_cell as usize].prev;

        self.cells[sp as usize].next = idx;
        self.cells[idx as usize] = CellNode { next: new_cell, prev: sp };
        self.cells[new_cell as usize].prev = idx;
        new_cell
    }

    fn update(&mut self, idx_: u32, pos: Vec2<i32>, cur_cell: u32) -> u32 {
        let (cx, cy) = (pos.0 as u32 >> CELL_SHIFT, (pos.1 as u32 >> CELL_SHIFT) << GRID_SHIFT);
        let new_cell = (cx + cy) & (CELL_MASK | (CELL_MASK << GRID_SHIFT));
        let idx = idx_ + GRID_SIZE;

        if cur_cell != new_cell {
            let (n, p) = (self.cells[idx as usize].next, self.cells[idx as usize].prev);
            self.cells[p as usize].next = n;
            self.cells[n as usize].next = p;

            let sp = self.cells[new_cell as usize].prev;

            self.cells[sp as usize].next = idx;
            self.cells[idx as usize].next = new_cell;
            self.cells[idx as usize].prev = sp;
            self.cells[new_cell as usize].prev = idx;
        }

        new_cell
    }

    fn swap_remove(&mut self, idx_: u32, last_idx_: u32) {
        let idx = idx_ + GRID_SIZE;
        let last_idx = last_idx_ + GRID_SIZE;

        {
            let (n, p) = (self.cells[idx as usize].next, self.cells[idx as usize].prev);
            self.cells[p as usize].next = n;
            self.cells[n as usize].prev = p;
        }

        let last = self.cells[last_idx as usize];
        self.cells[idx as usize] = last;
        self.cells[last.prev as usize].next = idx;
        self.cells[last.next as usize].prev = idx;
    }
}

pub struct CellphaseIterator<'a, T: 'a> {
    base: *const CellNode,
    data_base: *mut T,

    exclude: u32, // TODO: This should be offset by GRID_WIDTH*GRID_WIDTH
    n: u32,
    end: u32,

    xbeg: u32,
    x: u32, y: u32,
    xend: u32, yend: u32,

    phantom: PhantomData<&'a T>
}

impl<'a, T: 'a> Iterator for CellphaseIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        loop {
            unsafe {
                self.n = (*self.base.offset(self.n as isize)).next;
                if self.n != self.exclude {
                    if self.n != self.end {
                        return Some(&mut *self.data_base.offset((self.n - GRID_SIZE) as isize));
                    }

                    if self.x == self.xend {
                        if self.y == self.yend {
                            return None;
                        }
                        self.y += GRID_WIDTH;
                        self.x = self.xbeg;
                    }

                    self.x += 1;
                    self.n = (self.x + self.y) & (CELL_MASK | (CELL_MASK << GRID_SHIFT));
                    self.end = self.n;
                }
            }
        }
    }
}

// Brute-force broadphase

pub struct Bruteforce;

impl Broadphase for Bruteforce {
    fn new(_: u32) -> Bruteforce {
        Bruteforce
    }

    fn insert(&mut self, _: u32, _: Vec2<i32>) -> u32 {
        0
    }

    fn update(&mut self, _: u32, _: Vec2<i32>, _: u32) -> u32 {
        0
    }

    fn swap_remove(&mut self, _: u32, _: u32) {
        // Nothing
    }
}

pub trait ItemAdder<T: BroadphaseNode> {
    fn add(&mut self, item: T);
}

impl<T: BroadphaseNode, B: Broadphase> List<T, B> {
    
    pub fn new(max_count: u32) -> List<T, B> {
        List {
            items: Vec::with_capacity(max_count as usize),
            broadphase: <B as Broadphase>::new(max_count)
        }
    }

    pub fn iter_mut(&mut self) -> ListIterator<T, B> {
        ListIterator {
            list: self as *mut List<T, B>,
            cur: 0,
            phantom: PhantomData
        }
    }
}

impl<T: BroadphaseNode, B: Broadphase> ItemAdder<T> for List<T, B> {
    fn add(&mut self, mut item: T) {
        if self.items.len() < self.items.capacity() {
            //item.cell = self.broadphase.add(item.pos.as_i32());
            let cell_index = self.broadphase.insert(self.items.len() as u32, item.pos());
            item.set_cell(cell_index);
            self.items.push(item)
        }
    }
}

pub struct ListIterator<'a, T: 'a + BroadphaseNode, B: Broadphase> {
    list: *mut List<T, B>,
    cur: u32,
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

pub struct ListItemRef<'a, T: 'a + BroadphaseNode, B: Broadphase> {
    cur: &'a mut u32,
    list: *mut List<T, B>
}

impl<'a, T: 'a + BroadphaseNode, B: Broadphase> ListItemRef<'a, T, B> {
    pub fn value(&mut self) -> &mut T {
        unsafe { &mut (*self.list).items[*self.cur as usize] }
    }

    pub fn remove(self) {
        // Swap with last
        unsafe {
            let index = *self.cur;
            let list = &mut *self.list;

            list.items.swap_remove(index as usize);
            list.broadphase.swap_remove(index, list.items.len() as u32);
            
            mem::forget(self);
        }
    }

    pub fn update_broadphase(self) -> u32 {
        unsafe {
            let index = *self.cur;
            let list = &mut *self.list;
            let item = &mut list.items[index as usize];

            let new_cell = list.broadphase.update(index, item.pos(), item.get_cell());
            item.set_cell(new_cell);
            new_cell
        }
    }

    pub fn others_iter<'b>(&'b mut self) -> SubIterator<'b, T> {
        unsafe {
            let first = (&mut (*self.list).items[0]) as *mut T; // TODO: Better way to do this
            SubIterator {
                exclude: first.offset(*self.cur as isize),
                cur: first,
                end: first.offset((*self.list).items.len() as isize),
                phantom: PhantomData
            }
        }
    }
}

impl<'a, T: 'a + BroadphaseNode> ListItemRef<'a, T, Cellphase> {
    pub fn others_area_iter<'b>(&'b mut self, ul: Vec2<i32>, lr: Vec2<i32>) -> CellphaseIterator<'b, T> {

        let (list, base, data_base, exclude);
        unsafe {
            list = &mut *self.list;

            base = (&list.broadphase.cells[0]) as *const CellNode; // TODO: Better way to do this
            data_base = (&mut list.items[0]) as *mut T; // TODO: Better way to do this
            exclude = *self.cur + GRID_SIZE;
        }

        let (cx1, cy1) = (ul.0 as u32 >> CELL_SHIFT, (ul.1 as u32 >> CELL_SHIFT) << GRID_SHIFT);
        let (cx2, cy2) = (lr.0 as u32 >> CELL_SHIFT, (lr.1 as u32 >> CELL_SHIFT) << GRID_SHIFT);

        let first = (cx1 + cy1) & (CELL_MASK | (CELL_MASK << GRID_SHIFT));

        CellphaseIterator {
            base: base,
            data_base: data_base,
            exclude: exclude,
            n: first,
            end: first,
            x: cx1,
            xbeg: cx1 - 1,
            y: cy1,
            xend: cx2,
            yend: cy2,
            phantom: PhantomData
        }
    }
}

impl<'a, T: 'a + BroadphaseNode, B: Broadphase> Drop for ListItemRef<'a, T, B> {
    fn drop(&mut self) {
        *self.cur = *self.cur + 1;
    }
}

pub trait InteractIterator {
    type Item;
    type Broadphase;

    fn next<'a>(&'a mut self) -> Option<ListItemRef<'a, <Self as InteractIterator>::Item, <Self as InteractIterator>::Broadphase>>;
}

impl<'a, T: 'a + BroadphaseNode, B: Broadphase> InteractIterator for ListIterator<'a, T, B> {
    type Item = T;
    type Broadphase = B;

    fn next<'b>(&'b mut self) -> Option<ListItemRef<'b, T, B>> {
        unsafe {
            if self.cur != (*self.list).items.len() as u32 {
                Some(ListItemRef {
                    cur: &mut self.cur,
                    list: self.list
                })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use list::{List, Cellphase, Broadphase, BroadphaseNode, ItemAdder, InteractIterator};
    use fixed::Vec2;
    use std::num::Zero;

    #[derive(Copy, Clone)]
    struct Pixel {
        pos: Vec2<i32>,
        cell_index: u32
    }

    impl BroadphaseNode for Pixel {
        fn set_cell(&mut self, index: u32) {
            self.cell_index = index;
        }

        fn get_cell(&mut self) -> u32 {
            self.cell_index
        }

        fn pos(&self) -> Vec2<i32> {
            self.pos
        }
    }

    #[test]
    fn cellphase() {
        let mut bp: Cellphase = Broadphase::new(100);
        let v: Vec2<i32> = Zero::zero();
        let cell = bp.insert(0, v);
        let _ = bp.update(0, Vec2(100, 100), cell);
        bp.swap_remove(0, 0);
    }

    #[test]
    fn cellphase_list() {
        let mut bp: List<Pixel, Cellphase> = List::new(100);
        
        bp.add(Pixel { pos: Vec2(0, 0), cell_index: 0 });
        bp.add(Pixel { pos: Vec2(1, 1), cell_index: 0 });

        let mut i = bp.iter_mut();
        while let Some(mut n) = i.next() {
            {
                let val = n.value();
                val.pos = val.pos + Vec2(16, 0);
            }

            assert_eq!(n.update_broadphase(), 1);
        }
    }
}