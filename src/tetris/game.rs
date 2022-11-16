pub trait Game {
	fn reset(&mut self);
	fn run(&mut self) -> bool;
	fn score(&self) -> f32;
}
