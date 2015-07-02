use fixed::{Vec2, F64Vec};

pub struct Xorshift {
	x: u32
}

impl Xorshift {
	pub fn new() -> Xorshift {
		Xorshift { x: 0 }
	}

	pub fn next(&mut self) -> u32 {
		let mut v = self.x;
		v ^= v << 2;
		v ^= v >> 9;
		v ^= v << 15;
		self.x = v;
		v
	}

	pub fn next_max_f64(&mut self, max: f64) -> f64 {
		((self.next() as f64) * (1.0 / 4294967296.0)) * max
	}

	pub fn next_max_u32(&mut self, max: u32) -> u32 {
		self.next() % max
	}
	

	pub fn next_max_vec2(&mut self, max: f64) -> F64Vec {

		Vec2(
			self.next_max_f64(max * 2.0) - max,
			self.next_max_f64(max * 2.0) - max)
		/*
		let max_i = max.as_raw();

		Vec2(
			Fixed::from_raw(self.next_max(max_i as u32 * 2) as i32 - max_i),
			Fixed::from_raw(self.next_max(max_i as u32 * 2) as i32 - max_i))
		*/


	}
}

/*
pub trait Rand {
	fn rand(rng: &mut Xorshift) -> Self;
}

impl<T: Copy + Rand> Rand for Vec2<T> {
	fn rand(rng: &mut Xorshift) -> Self {
		Vec2(Rand::rand(rng), Rand::rand(rng))
	}
}*/
