use std::mem;

#[repr(C)]
pub struct Sim {
	pub x: Vec<u32>,
	pub y: Vec<u32>,
	pub z: Vec<u32>
}

#[repr(C)]
pub struct SimPrivX {
	_x: Vec<u32>,
	pub y: Vec<u32>,
	pub z: Vec<u32>
}

impl Sim {
	fn split_on_x(&mut self) -> (&mut SimPrivX, &mut Vec<u32>) {
		unsafe { (mem::transmute(self as *mut Sim), &mut self.x) }
	}
}

#[cfg(test)]
mod tests {
	use super::Sim;

	#[test]
	fn parallel_access() {
		let mut sim = Sim { x: Vec::new(), y: Vec::new(), z: Vec::new() };
		
		{
			let (mut sim_minus_x, mut x) = sim.split_on_x();

			x.push(1);
			sim_minus_x.y.push(2);
		}

		assert_eq!(sim.x[0], 1);
		assert_eq!(sim.y[0], 2);
	}
}