use fixed::Vec2;

#[derive(Copy, Clone)]
pub struct Mat(u8);

pub struct Level {
    width: u32,
    height: u32,
	materials: Vec<Mat>,

}



const DIRT: u8 = 1 << 0;
const DIRT2: u8 = 1 << 1;
const ROCK: u8 = 1 << 2;
const BACK: u8 = 1 << 3;
const SEE_SHADOW: u8 = 1 << 4;
const WORM_M: u8 = 1 << 5;

impl Level {
	pub fn new() -> Level {
        let width = 504;
        let height = 350;
		Level {
            width: width,
            height: height,
			materials: vec![Mat(BACK); (height * width) as usize]
		}
	}

    pub fn mat(&self, pos: Vec2<i32>) -> Mat {
    	if pos.0 >= 0 && pos.0 < self.width as i32
    	&& pos.1 >= 0 && pos.1 < self.height as i32 {
			self.materials[(pos.1 * self.width as i32 + pos.0) as usize]
    	} else {
    		Mat(ROCK)
    	}
    }
}

impl Mat {
	#[inline] pub fn dirt(self) -> bool { self.0 & DIRT != 0 }
    #[inline] pub fn dirt2(self) -> bool { self.0 & DIRT2 != 0 }
    #[inline] pub fn rock(self) -> bool { self.0 & ROCK != 0 }
    #[inline] pub fn back(self) -> bool { self.0 & BACK != 0 }
    #[inline] pub fn see_shadow(self) -> bool { self.0 & SEE_SHADOW != 0 }
    #[inline] pub fn dirt_rock(self) -> bool { self.0 & (DIRT | DIRT2 | ROCK) != 0 }

    #[inline] pub fn any_dirt(self) -> bool { self.0 & (DIRT | DIRT2) != 0 }
    #[inline] pub fn dirt_back(self) -> bool { self.0 & (DIRT | DIRT2 | BACK) != 0 }
    #[inline] pub fn worm(self) -> bool { self.0 & WORM_M != 0 }
}