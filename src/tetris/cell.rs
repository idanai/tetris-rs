use std::fmt;
use super::tetromino::Shape;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Full(Shape),
	Ghost(Shape),
}

//#[derive(Copy, Clone, Default)]
//struct Cell(Option<Shape>);

impl Default for Cell {
	fn default() -> Self {
		Self::Empty
	}
}

impl Cell {
	pub fn empty(&mut self) {
		*self = Self::Empty;
	}

	pub fn to_ghost(&mut self) {
		*self = Cell::Ghost(self.unwrap());
	}

	pub fn to_full(&mut self) {
		*self = Cell::Full(self.unwrap());
	}

	// instead of creating closures every time
	pub fn is_full(&self) -> bool {
		matches!(self, Self::Full(_))
	}

	pub fn is_ghost(&self) -> bool {
		matches!(self, Self::Ghost(_))
	}

	pub fn is_empty(&self) -> bool {
		matches!(self, Self::Empty)
	}

	pub fn unwrap(&self) -> Shape {
		use Cell::*;
		match self {
			Full(shape) | Ghost(shape) => *shape,
			Empty => panic!("Can't unwrap Cell::Empty. Can only unwrap Cell::Full & Cell::Ghost"),
		}
	}
}

impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Cell::*;
		use termion::color::*;
//		let color = match self {
//			Full(shape) => shape.fg_color_str(),
//			Ghost(_) => White.fg_str(),
//			Empty => Black.fg_str(),
//		};
//		write!(f, "{}██", color)
		match self {
			Full(shape) => write!(f, "{}██", shape.fg_color_str()),
			Ghost(_) => write!(f, "{}░▒", White.fg_str()),// • ░▒▓█▀▄≡■ 
			Empty => write!(f, "{}██", Black.fg_str()),
		}
	}
}
