use fixed::{Fixed, FixedVec};

#[derive(Copy, Clone)]
pub struct Worm {
	pub pos: FixedVec,
	pub vel: FixedVec,
	pub aiming_angle: Fixed,
	pub aiming_vel: Fixed,
}

/*

fixedvec pos, vel;

gvl::ivec2 logicRespawn;

int hotspotX, hotspotY;      //Hotspots for laser, laser sight, etc.
fixed aimingAngle, aimingSpeed;

//Controls controls;
bool ableToJump, ableToDig;   //The previous state of some keys
bool keyChangePressed;
bool movable;

bool animate;                 //Should the worm be animated?
bool visible;                 //Is the worm visible?
bool ready;                   //Is the worm ready to play?
bool flag;                    //Does the worm have a flag?
bool makeSightGreen;          //Changes the sight color
int health;                  //Health left
int lives;                   //lives left
int kills;                   //Kills made

int timer;                   //Timer for GOT
int killedTimer;             //Time until worm respawns
int currentFrame;

int flags;                   //How many flags does this worm have?

Ninjarope ninjarope;

int currentWeapon;           //The selected weapon
int lastKilledByIdx;          // What worm that last killed this worm
int fireCone;                //How much is left of the firecone
int leaveShellTimer;         //Time until next shell drop

*/