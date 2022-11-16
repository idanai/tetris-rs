use std::fmt;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct V2 {
	pub x : i32,
	pub y : i32,
}

impl V2 {
	pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
}

impl std::ops::Add for V2 {
	type Output = Self;
	fn add(self, other: Self) -> V2 {
		V2::new(self.x + other.x, self.y + other.y)
	}
}

impl std::ops::Sub for V2 {
	type Output = Self;
	fn sub(self, other: Self) -> V2 {
		V2::new(self.x - other.x, self.y - other.y)
	}
}

impl std::ops::Mul<i32> for V2 {
	type Output = Self;
	fn mul(self, coef: i32) -> V2 {
		V2::new(self.x * coef, self.y * coef)
	}
}

impl std::ops::Div<i32> for V2 {
	type Output = Self;
	fn div(self, coef: i32) -> V2 {
		V2::new(self.x / coef, self.y / coef)
	}
}

impl fmt::Display for V2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "({}, {})", self.x, self.y)
    }
}

//impl Add for V2 {
//	fn add(&self, other : &Self) -> Self {
//		Self {
//			x : self.x + other.x,
//			y : self.y + other.y
//		}
//	}
//}
//
//impl Sub for V2 {
//	fn sub(&self, other : &Self) -> Self {
//		Self {
//			x : self.x - other.x,
//			y : self.y - other.y
//		}
//	}
//}
//
