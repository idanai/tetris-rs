// Tetromino = shape composed of 4 cells connected orthogonally

pub mod offsets;
pub mod data;

//use crate::tetris::v2::*;
//use direction::*;
use offsets::Offsets;
use rand::{
	distributions::{Distribution, Standard},
	Rng
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Shape { I, O, T, L, J, S, Z }

impl Shape {
	pub fn fg_color_str(&self) -> &'static str {
		use Shape::*;
		use termion::color::*;
		match self {
			I => Cyan.fg_str(),
			O => Yellow.fg_str(),
			T => Magenta.fg_str(),
			L => LightBlue.fg_str(),
			J => Blue.fg_str(),
			S => Green.fg_str(),
			Z => Red.fg_str(),
		}
	}
}

impl Distribution<Shape> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
		use Shape::*;
		match rng.gen_range(0..24) { // same as rand(6) + rand(6)?
			0 => I,
			1..=2 => O,
			3..=5 => T,
			6..=9 => L,
			10..=13 => J,
			14..=18 => S,
			19..=23 => Z,
			_ => panic!("Random number generated outisde what i wanted")
		}
	}
}


#[derive(Copy, Clone)]
enum Side { Left = -1, Right = 1 }

// tetromino is the name of a tetris piece consisting of four orthogonally connected squares
#[derive(Copy, Clone)]
pub struct Tetromino {
	data: &'static data::ShapeData,
	state: i8, // there are only four states, usize is unnecessary
}

impl PartialEq for Tetromino {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for Tetromino{}

impl Tetromino {
	pub fn rotate_left(&mut self) {
		self.rotate(Side::Left);
	}

	pub fn rotate_right(&mut self) {
		self.rotate(Side::Right);
	}

	// get current rotation
	pub fn current_offsets(&self) -> &Offsets {
		&self.data.rotations[self.state as usize]
	}

	// get current shape- each of it's cells' possible colliding direction
	pub fn current_directions(&self) -> &data::MovementFlags {
		&self.data.directions[self.state as usize]
	}

	// todo add 'make sure it never panics panic! macro'
	pub fn indexes_of_cells_colliding(&self, direction: data::Flags) -> Vec<usize> {
//		let mut iter = Iterator::zip(self.current_shape().iter(), self.current_directions().decompress().iter());
//		iter.filter_map(|(cell_offset, cell_dir)| cell_dir.can_collide(direction).then_some(cell_offset.clone()))
		self.current_directions()
		.decompress().iter()
		.enumerate()
		.filter_map(
			|(index, cell)| cell.can_collide(direction).then_some(index)
		)
		.collect()
	}

//	pub fn cells_colliding(&self, direction: data::Flags) -> impl Iterator<Item=&V2> + '_ {
//		let mut r_iter = self.data[self.state as usize].rotations.iter();
//		let mut d_iter = self.data.directions.iter();
//		let mut zip_iter = Iterator::zip(r_iter, d_iter);
//		zip_iter.filter_map(|(rot, dir)| if dir.can_move(direction) {Some(rot)} else {None})
//	}

	fn rotate(&mut self, side: Side) {
		self.state = (self.state + side as i8).rem_euclid(self.current_offsets().len() as i8);
//		self.state += if matches!(side, Side::Left) {-1} else {1};
//		self.state %= self.current_offsets().len();
	}
}

impl From<Shape> for Tetromino {
	fn from(t: Shape) -> Self {
		use Shape::*;
		Self {
			data : match t{
				I => &data::I,
				O => &data::O,
				T => &data::T,
				L => &data::L,
				J => &data::J,
				S => &data::S,
				Z => &data::Z,
			},
			state: 0
		}
	}
}
