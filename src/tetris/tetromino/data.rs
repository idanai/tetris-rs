use crate::tetris::v2::V2;
use super::offsets::Offsets;

// Holds all variants of a Tetromino
pub struct ShapeData {
	pub rotations: &'static [Offsets], // all possible rotations
	pub directions: &'static [MovementFlags], // the direction each cell can move in each rotations
}

#[derive(Copy, Clone)]
pub struct Flags(u8);

impl Flags{
	pub fn can_collide(&self, f: Self) -> bool {
		f.0 == f.0 & self.0	
	}
}

impl std::ops::BitOr for Flags {
	type Output = Flags;
	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

use crate::tetris::direction::Direction;

impl From<Direction> for Flags {
	fn from(d: Direction) -> Self {
		match d{
			Direction::Up => Flags(UP),
			Direction::Down => Flags(DOWN),
			Direction::Left => Flags(LEFT),
			Direction::Right => Flags(RIGHT),
		}
	}
}

// instead of enum
//pub const UNMOVING: Flags = Flags(0);
//pub const DOWN: Flags = Flags(1);
//pub const LEFT: Flags = Flags(2);
//pub const RIGHT: Flags = Flags(4);
//pub const UP: Flags = Flags(8);
//pub const HORIZONTAL: Flags = Flags(6);
//pub const VERTICAL: Flags = Flags(9);
//pub const All: Flags = Flags(0x0F);
pub const UNMOVING: u8 = 0;
pub const DOWN: u8 = 1;
pub const LEFT: u8 = 2;
pub const RIGHT: u8 = 4;
pub const UP: u8 = 8;
pub const HORIZONTAL: u8 = 6;
pub const VERTICAL: u8 = 9;
pub const ALL: u8 = 0x0F;

/*
// todo use the already existing Direction enum
#[derive(Copy, Clone)]
enum Direction {
	None,
	DOWN,
	LEFT,
	RIGHT,
	UP,
	HORIZONTAL,
	VERTICAL,
	All,
}

use Direction::*;

impl std::ops::BitOr for Direction {
	type Output = Flags;
	fn bitor(self, rhs: Rhs) -> Self::Output {
		Flags::from(self) | rhs
	}
}

impl From<Direction> for Flags {
	fn from(d: Direction) -> Self {
		Self(match self {
			None => 0,
			DOWN => 1,
			LEFT => 2,
			RIGHT => 4,
			UP => 8,
			HORIZONTAL => 2 | 4,
			VERTICAL 1 | 8,
			All => 0x0F,
		})
	}
}


impl std::ops::BitOr<Rhs: Direction> for Flags {
	type Output = Flags;
	fn bitor(self, rhs: Rhs) -> Self::Output {
		self | Self::from(rhs)
	}
}
*/


// used for collision checking for each cell
// 16 bits; 4 bits (flags) for each tetromino
// each bit represents if a cell of a tetromino can move in a certain direction
//   A
// B C D
//
// 16 bits = A B C D (4 bits per cell)
pub struct MovementFlags {
	flags: u16,
}

impl MovementFlags {
	const fn new(four_flags: &[u8; 4]) -> Self {
		let mut flags = 0u16;

		flags |= (four_flags[0] & 0x0F) as u16;
		flags |= ((four_flags[1] & 0x0F) as u16) << 4;
		flags |= ((four_flags[2] & 0x0F) as u16) << 8;
		flags |= ((four_flags[3] & 0x0F) as u16) << 12;

//		for f in four_flags.iter().rev() {
//			flags |= (0x0F & *f) as u16; // todo ? just in case maybe add 0x0F &
//			flags <<= 4;
//		}

		Self {flags}
	}

	pub unsafe fn unchecked_at(&self, index: usize) -> Flags {
		let f = 0x0F & (self.flags >> (index as u16 * 4)) as u8;
		Flags(f)
	}

	pub fn at(&self, index: usize) -> Flags {
		if index >= 4 {
			panic!("Can't index MovementFlags with a number greater than 3");
		}
		unsafe { self.unchecked_at(index) }
	}

	pub fn decompress(&self) -> [Flags; 4] {
		let mut flags = [Flags(UNMOVING); 4]; // todo check how to do it faster, without initializing to UNMOVING
		unsafe {
			flags.iter_mut().enumerate().for_each(|(index, item)| *item = self.unchecked_at(index));
//			(0..4).for_each(|index| flags[index] = self.unchecked_at(index)).collect();
		}
		flags
	}
}


// All possible phases of allthe tetrominos
pub static I: ShapeData = ShapeData {
	rotations: &[
		// * x * *
		//
		Offsets([
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 2,y: 0},
		]),
		// *
		// x
		// *
		// *
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
			V2{x: 0,y: 2},
		]),
		//
		// * x * *
		Offsets([
			V2{x:-1,y: 1},
			V2{x: 0,y: 1},
			V2{x: 1,y: 1},
			V2{x: 2,y: 1},
		]),
		// _ *
		// _ x
		// _ *
		// _ *
		Offsets([
			V2{x: 1,y:-1},
			V2{x: 1,y: 0},
			V2{x: 1,y: 1},
			V2{x: 1,y: 2},
		]),
	],
	directions: &[
		MovementFlags::new(&[LEFT | VERTICAL, VERTICAL, VERTICAL, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, HORIZONTAL, HORIZONTAL, HORIZONTAL | DOWN]),
		MovementFlags::new(&[LEFT | VERTICAL, VERTICAL, VERTICAL, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, HORIZONTAL, HORIZONTAL, HORIZONTAL | DOWN]),
	]
};

pub static O: ShapeData = ShapeData{
	rotations: &[
		// x *
		// * *
		Offsets([
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 0,y: 1},
			V2{x: 1,y: 1},
		])
	],
	directions: &[
		MovementFlags::new(&[LEFT | UP, UP | RIGHT, LEFT | DOWN, DOWN | RIGHT])
	]
};

pub static T: ShapeData = ShapeData{
	rotations: &[
		// * x *
		//   *
		Offsets([
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 0,y: 1},
		]),
		// *
		// x *
		// *
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 0,y: 1},
		]),
		//   *
		// * x *
		Offsets([
			V2{x: 0,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
		]),
		//   *
		// * x
		//   *
		Offsets([
			V2{x: 0,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
		]),
	],
	directions: &[
		MovementFlags::new(&[LEFT | VERTICAL, UP, VERTICAL | RIGHT, HORIZONTAL | DOWN]),
		MovementFlags::new(&[HORIZONTAL | UP, LEFT, VERTICAL | RIGHT, HORIZONTAL | DOWN]),
		MovementFlags::new(&[UP | HORIZONTAL, LEFT | VERTICAL, DOWN, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, LEFT | VERTICAL, RIGHT, HORIZONTAL | DOWN]),
	]
};

pub static L: ShapeData = ShapeData{
	rotations: &[
		// * x *
		// *
		Offsets([
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x:-1,y: 1},
		]),
		// *
		// x
		// * *
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
			V2{x: 1,y: 1},
		]),
		//     *
		// * x *
		Offsets([
			V2{x: 1,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
		]),
		// * *
		//   x
		//   *
		Offsets([
			V2{x:-1,y:-1},
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
		]),
	],
	directions: &[
		MovementFlags::new(&[UP | LEFT, VERTICAL, VERTICAL | RIGHT, HORIZONTAL | DOWN]),
		MovementFlags::new(&[UP | HORIZONTAL, HORIZONTAL, LEFT | DOWN, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, LEFT | VERTICAL, VERTICAL, DOWN | RIGHT]),
		MovementFlags::new(&[LEFT | VERTICAL, UP | RIGHT, HORIZONTAL, HORIZONTAL | DOWN]),
	]
};

pub static J: ShapeData = ShapeData{
	rotations: &[
		// * x *
		//     *
		Offsets([
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 1,y: 1},
		]),
		// * *
		// x
		// *
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 1,y:-1},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
		]),
		// *
		// * x *
		Offsets([
			V2{x:-1,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
		]),
		//   *
		//   x
		// * *
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x:-1,y: 1},
			V2{x: 0,y: 1},
		]),
	],
	directions: &[
		MovementFlags::new(&[LEFT | VERTICAL, VERTICAL, UP | RIGHT, HORIZONTAL | DOWN]),
		MovementFlags::new(&[UP | LEFT, VERTICAL | RIGHT, HORIZONTAL, HORIZONTAL | DOWN]),
		MovementFlags::new(&[HORIZONTAL | UP, LEFT | DOWN, VERTICAL, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, HORIZONTAL, LEFT | VERTICAL, DOWN | RIGHT]),
	]
};

pub static S: ShapeData = ShapeData{
	rotations: &[
		//   x *
		// * *
		Offsets([
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x:-1,y: 1},
			V2{x: 0,y: 1},
		]),
		// *
		// x *
		//   *
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 1,y: 1},
		]),
		//   * *
		// * x
		Offsets([
			V2{x: 0,y:-1},
			V2{x: 1,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
		]),
		// *
		// * x
		//   *
		Offsets([
			V2{x:-1,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
		]),
	],
	directions: &[
		MovementFlags::new(&[UP | LEFT, VERTICAL | RIGHT, LEFT | VERTICAL, DOWN | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, LEFT | DOWN, UP | RIGHT, HORIZONTAL | DOWN]),
		MovementFlags::new(&[UP | LEFT, VERTICAL | RIGHT, LEFT | VERTICAL, DOWN | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, LEFT | DOWN, UP | RIGHT, HORIZONTAL | DOWN]),
	]
};

pub static Z: ShapeData = ShapeData{
	rotations: &[
		// * x
		//   * *
		Offsets([
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x: 0,y: 1},
			V2{x: 1,y: 1},
		]),
		//   *
		// x *
		// *
		Offsets([
			V2{x: 1,y:-1},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
			V2{x: 0,y: 1},
		]),
		// * *
		//   x *
		Offsets([
			V2{x:-1,y:-1},
			V2{x: 0,y:-1},
			V2{x: 0,y: 0},
			V2{x: 1,y: 0},
		]),
		//   *
		// * x
		// *
		Offsets([
			V2{x: 0,y:-1},
			V2{x:-1,y: 0},
			V2{x: 0,y: 0},
			V2{x:-1,y: 1},
		]),
	],
	directions: &[
		MovementFlags::new(&[LEFT | VERTICAL, UP | RIGHT, LEFT | DOWN, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, UP | LEFT, DOWN | RIGHT, HORIZONTAL | DOWN]),
		MovementFlags::new(&[LEFT | VERTICAL, UP | RIGHT, LEFT | DOWN, VERTICAL | RIGHT]),
		MovementFlags::new(&[UP | HORIZONTAL, UP | LEFT, DOWN | RIGHT, HORIZONTAL | DOWN]),
	]
};
