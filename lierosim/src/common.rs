use nobject::NObjectType;
use fixed::{Vec2, F64Vec};
use std::num::Zero;


pub struct Common {
    pub nobject_types: [NObjectType; 40 + 25],
    pub sincos: [F64Vec; 128]
}

#[derive(Copy, Clone, Debug)]
struct FP(i64, i32);

impl FP {

	fn as_f64(self) -> f64 {
		let FP(s, bits) = self;

		(s as f64) / ((1i64 << bits) as f64)
	}

	fn reduce(&mut self, tobits: i32) {
		let lim = 1i64 << tobits;

		while self.0 < (-lim - 1) || self.0 > lim {
			self.0 >>= 1;
			self.1 -= 1;
		}
	}

	fn reduced_frac(self, tobits: i32) -> i64 {
		let FP(mut rs, mut rbits) = self;

		while rbits > 58 {
			rs >>= 1;
			rbits -= 1;
		}

		rs << (tobits - rbits)
	}
}

fn init_sincos2(sincos: &mut [F64Vec]) {
	let scale = 0.04908738521234052; // (2pi / 128)

	for i in 0i32..128i32 {
		let mut rf = 0f64;
		let mut c = -1f64;
		let xf = i as f64 * scale;

		let mut num = xf;

		for t in 1..13 {
			rf += c * num;

			num = (num * xf) / (2 * t) as f64;
			num = (num * xf) / (2 * t + 1) as f64;
			
			c = -c;
		}

		let r = rf;
		//let r = Fixed::from_raw(0);

		sincos[i as usize].0 = r;
		sincos[((i + 32) & 0x7f) as usize].1 = r;
	}
}

/*
fn init_sincos(sincos: &mut [FixedVec]) {
	let scalebits = 28;
	let scale = 13176795; // (2pi / 128) << scalebits

	for i in 75..128 {
		let mut rf = 0i64;
		let mut c = -1i64;
		let xf: i32 = i * scale;

		println!("xf = {}", (xf as f64) / (1i32 << scalebits) as f64);

		let mut num = FP(xf as i64, scalebits);

		for t in 1..13 {
			//println!("{:?}", num);
			rf += c * num.reduced_frac(58);
			//println!("+ {}", c * num.reduced_frac(59));
			//println!("a: {}", num.as_f64());
			num.0 = num.0 / (2 * t);
			//println!("b: {}", num.as_f64());
			num.reduce(31);
			//println!("c: {}", num.as_f64());
			num.0 = num.0 * xf as i64;
			num.1 = num.1 + scalebits;
			//println!("red1: {}, xf = {}", num.as_f64(), (xf as f64) / (1i32 << scalebits) as f64);
			
			num.0 = num.0 / (2 * t + 1);
			//println!("d: {}", num.as_f64());
			num.reduce(31);
			//println!("e: {}", num.as_f64());
			num.0 = num.0 * xf as i64;
			num.1 += scalebits;
			//println!("red2: {}", num.as_f64());

			c = -c;
		}

		let shift = 58 - 16;

		rf += 1i64 << (shift - 1); // Correct rounding
		let r = Fixed::from_raw((rf >> shift) as i32);

		sincos[i as usize].0 = r;
		sincos[((i + 32) & 0x7f) as usize].1 = r;
	}
}
*/

impl Common {
	pub fn new() -> Common {
		let mut c = Common {
			nobject_types: [NObjectType {
                gravity: 0.0,
                splinter_type: 0,
                splinter_count: 0,
                time_to_live: 0,
                time_to_live_v: 1
            }; 40 + 25],
			sincos: [Vec2(Zero::zero(), Zero::zero()); 128],
		};

		init_sincos2(&mut c.sincos);

		c
	}

	
}

/*

*/