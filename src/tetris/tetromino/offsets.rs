use crate::tetris::v2::V2;

// Each item is an offset, that when put together can represent a tetromino- a shape composed of 4 cells orthogonally connected
#[derive(Clone, Copy)]
pub struct Offsets(pub [V2; 4]);

// todo check if works
impl std::ops::Deref for Offsets {
	type Target = [V2];
	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}
