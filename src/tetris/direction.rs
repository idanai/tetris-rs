use super::v2::*;

#[derive(Copy, Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl From<Direction> for V2 {
	fn from(d: Direction) -> V2 {
		use Direction::*;
		match d {
			Up => V2::new(0,-1),
			Down => V2::new(0,1),
			Left => V2::new(-1,0),
			Right => V2::new(1,0),
		}
	}
}
