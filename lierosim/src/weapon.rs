use fixed::{F64Vec};
use worm::Worm;
use sim::SimMinusWorms;
use common::Common;

struct WeaponType {
	obj_idx: u32,
	parts: u32,
	speed: f64,
	affect_by_worm: f64,
}

impl WeaponType {
	fn fire_particle(
		&self,
		angle: i32,
		pos: F64Vec,
		vel: F64Vec,
		speed: f64,
		state: &mut SimMinusWorms,
		common: &Common
		//int angle, fixedvec vel, int speed, fixedvec pos, int ownerIdx, WormWeapon* ww
		) {
			let combined_vel = common.sincos[angle as usize] * speed
				+ vel
				+ state.rand().next_max_vec2(0.01); // TODO: distribution from parent

			state.new_nobject(self.obj_idx, pos, combined_vel);

		/*

		WObject* obj = game.wobjects.newObjectReuse();
		Common& common = *game.common;

		obj->type = this;
		obj->pos = pos;
		obj->ownerIdx = ownerIdx;

		// STATS
		obj->firedBy = ww;
		obj->hasHit = false;

		LTRACE(rand, 0, wobj, game.rand.x);
		LTRACE(fire, obj - game.wobjects.arr, cxpo, pos.x);
		LTRACE(fire, obj - game.wobjects.arr, cypo, pos.y);

		Worm* owner = game.wormByIdx(ownerIdx);
		game.statsRecorder->damagePotential(owner, ww, hitDamage);
		game.statsRecorder->shot(owner, ww);

		obj->vel = cossinTable[angle] * speed / 100 + vel;

		if(distribution)
		{
			obj->vel.x += game.rand(distribution * 2) - distribution;
			obj->vel.y += game.rand(distribution * 2) - distribution;
		}

		if(startFrame >= 0)
		{
			if(shotType == STNormal)
			{
				if(loopAnim)
				{
					if(numFrames)
						obj->curFrame = game.rand(numFrames + 1);
					else
						obj->curFrame = game.rand(2);
				}
				else
					obj->curFrame = 0;
			}
			else if(shotType == STDType1)
			{
				if(angle > 64)
					--angle;
					
				int curFrame = (angle - 12) >> 3;
				if(curFrame < 0)
					curFrame = 0;
				else if(curFrame > 12)
					curFrame = 12;
				obj->curFrame = curFrame;
			}
			else if(shotType == STDType2 || shotType == STSteerable)
			{
				obj->curFrame = angle;
			}
			else
				obj->curFrame = 0;
		}
		else
		{
			obj->curFrame = colorBullets - game.rand(2);
		}

		obj->timeLeft = timeToExplo;

		if(timeToExploV)
			obj->timeLeft -= game.rand(timeToExploV);
		*/
	}

	fn fire(&self, worm: &mut Worm, state: &mut SimMinusWorms, common: &Common) {

		let firing_vel = worm.vel * self.affect_by_worm;
		let aiming_angle = 0; // TODO

		for _ in 0..self.parts {
			self.fire_particle(
				aiming_angle,
				worm.pos,
				firing_vel,
				self.speed,
				state,
				common);
		}


		/*
		int speed = w.speed;
		fixedvec firingVel;
		int parts = w.parts;

		if(w.affectByWorm)
		{
			if(speed < 100)
				speed = 100;
			
			firingVel = vel * 100 / speed;
		}

		for(int i = 0; i < parts; ++i)
		{
			w.fire(
				game,
				ftoi(aimingAngle),
				firingVel,
				speed,
				firing,
				index, &ww);
		}
		
		int recoil = w.recoil;
		
		if(common.H[HSignedRecoil] && recoil >= 128)
			recoil -= 256;
		
		vel -= cossinTable[ftoi(aimingAngle)] * recoil / 100;
		*/
	}
}

