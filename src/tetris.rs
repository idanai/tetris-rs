pub mod game;
mod v2;
mod cell;
mod direction;
mod tetromino;

use std::fmt;

use game::Game;
use v2::*;
use cell::*;
use direction::*;
use tetromino::{Tetromino, Shape};

use std::io::{Read, Write};
use std::time;


const USER_INPUT_INTERVAL : time::Duration = time::Duration::from_millis(5);
const GAME_GRAVITY_INTERVAL : time::Duration = time::Duration::from_secs(1);
const GAME_DRAW_INTERVAL : time::Duration = time::Duration::from_millis(1000 / 24);

const T_SPIN_SCORE: u32 = 400;


#[derive(Clone, PartialEq, Eq)]
struct Piece {
	pos: V2,
	points: [V2; 4],
	data: Tetromino,
	cell_value: Cell,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        f.debug_struct("Piece")
//         .field("points", &self.points)
//         .finish()
         write!(f, "Points: [")?;
         for p in self.points {
         	write!(f, "{}", p)?;
         }
         write!(f, "]")
    }
}

impl Default for Piece {
	fn default() -> Self {
		Self {
			pos: V2::new(0,0),
			points: [V2::new(0,0); 4],
			data: Tetromino::from(Shape::I),
			cell_value: Cell::Ghost(Shape::I),
		}
	}
}

impl Piece {
	pub fn new(pos: V2, cell_value: Cell) -> Self {
		let data = Tetromino::from(cell_value.unwrap());
		let offsets = data.current_offsets();
		Self {
			pos,
			points: [pos + offsets[0], pos + offsets[1], pos + offsets[2], pos + offsets[3]],
			data,
			cell_value,
		}
	}

	pub fn clone_as_ghost(&self) -> Self {
		let mut copy = self.clone();
		copy.cell_value.to_ghost();
		copy
	}

	pub fn clone_as_full(&self) -> Self {
		let mut copy = self.clone();
		copy.cell_value.to_full();
		copy
	}

	pub fn translate(&mut self, v: V2) {
		self.pos = self.pos + v;
		self.points.iter_mut().for_each(|p| *p = *p + v);
	}

	// moves the coordinates of the piece by adding a V2 to each
	pub fn translated(&self, v: V2) -> Self {
		let mut copy = self.clone();
		copy.translate(v);
		copy
	}

	fn rotated(&self, is_left: bool) -> Self {
		let mut data = self.data.clone();
		if is_left {
			data.rotate_left();
		} else {
			data.rotate_right();
		}
		let pos = self.pos;
		let offsets = data.current_offsets();
		Self {
			pos,
			points: [pos + offsets[0], pos + offsets[1], pos + offsets[2], pos + offsets[3]],
			data,
			cell_value: self.cell_value,
		}
	}

	pub fn rotated_left(&self) -> Self {
		self.rotated(true)
	}

	pub fn rotated_right(&self) -> Self {
		self.rotated(false)
	}
}

const FALLER_INDEX: usize = 0;
const GHOST_INDEX: usize = 1;

pub struct Tetris<R, W> {
	width: usize, // todo can use u8
	height: usize, // todo can use u8
	map: Vec<Cell>,
	score: u32,
	game_over: bool,

	pieces: [Piece; 2], // unfortunately I have to get to pieces via index instead of reference. Damn you rust
//	piece: RefCell<Piece>, // data of the currently falling piece
//	ghost: RefCell<Piece>, // data of the future location of the currently falling piece

	now: time::Instant,
	start_time: time::Instant,
	next_frame_time: time::Instant,
	next_gravity_time: time::Instant,
	next_input_time: time::Instant,
	display_changed: bool,

	output: W,
	input: R,
}


// self.piece functionality
impl<R: Read, W: Write> Tetris<R, W> {
	fn faller(&self) -> &Piece {
		&self.pieces[FALLER_INDEX]
	}

	fn ghost(&self) -> &Piece {
		&self.pieces[GHOST_INDEX]
	}

	fn faller_mut(&mut self) -> &mut Piece {
		&mut self.pieces[FALLER_INDEX]
	}

	fn ghost_mut(&mut self) -> &mut Piece {
		&mut self.pieces[GHOST_INDEX]
	}

	fn fill_piece_cells_with(&mut self, piece_inedx: usize, value: Cell) {
		for point in self.pieces[piece_inedx].points {
			*self.at_mut(point) = value;
		}
	}

	fn remove_piece(&mut self, piece_inedx: usize) {
		self.fill_piece_cells_with(piece_inedx, Cell::Empty);
	}

	fn insert_piece(&mut self, piece_inedx: usize) {
		self.fill_piece_cells_with(piece_inedx, self.pieces[piece_inedx].cell_value);
	}

	fn remove_ghost(&mut self) {
		let g = &self.ghost();
		if g.pos != self.faller().pos {
			for p in g.points {
				let c = self.at_mut(p);
				if let Cell::Ghost(_) = *c {
					*c = Cell::Empty;
				}
			}
		}
	}

	fn update_piece(&mut self, piece_inedx: usize, new_piece: Piece, update_ghost: bool) {
		self.remove_piece(piece_inedx);
		self.pieces[piece_inedx] = new_piece;
		self.insert_piece(piece_inedx);
		if update_ghost {
			self.update_ghost(true);
		}
	}

	fn update_ghost(&mut self, try_remove: bool) {
		// Damnit, rust! so many indexing operations!!!
		// todo Should I consider copying instead?
		if try_remove {
			self.remove_ghost();
		}
//		if try_remove && self.ghost().pos != self.faller().pos { // 2 indexings here
//			self.remove_piece(GHOST_INDEX); // 1 indexing here
//		}
		*self.ghost_mut() = self.faller().clone_as_ghost(); // 2 indexings here
		self.throw_piece(GHOST_INDEX, Direction::Down); // 1 indexing here
		if self.ghost().pos != self.faller().pos { // 2 indexings here
			self.insert_piece(GHOST_INDEX); // 1 indexing here
		}
	}

	// moves the piece by the direction parameter
	// returns false if cannot move the piece
	fn try_move_piece(&mut self, piece: &mut Piece, d: Direction) -> bool {
		let offset = V2::from(d);
		let moved = piece.translated(offset);
		for index in moved.data.indexes_of_cells_colliding(d.into()) {
			let pos = moved.points[index];
			if !self.bounds_contain(pos) || self.at(pos).is_full() {
				return false;
			}
		}
		*piece = moved;
		true
	}

	fn try_move_piece_and_update(&mut self, piece_index: usize, d: Direction) -> bool {
		let mut piece = self.pieces[piece_index].clone();
		if self.try_move_piece(&mut piece, d) { // make the function return a moved named Piece
			self.update_piece(piece_index, piece, !matches!(d, Direction::Down | Direction::Up));
			return true;
		}
		false
	}
}

impl<R: Read, W: Write> Tetris<R, W> {
	pub fn new(width : usize, height : usize, output : W, input : R) -> Self {
		let now = time::Instant::now();
		let temp = Piece::new(V2::new(width as i32 / 2, 0), Cell::Full(Shape::I));
		let temp_ghost = temp.clone_as_ghost();
		Self {
			width, height,
			map: vec![Default::default(); width * height],
			score: 0,
			game_over: true,
			pieces: [temp, temp_ghost],
			now,
			start_time: now,
			next_frame_time: now,
			next_gravity_time: now,
			next_input_time: now,
			display_changed: true,
			output,
			input,
		}
	}

	fn at_mut(&mut self, p: V2) -> &mut Cell{
		&mut self.map[p.x as usize + p.y as usize * self.width]
	}

	fn at(&self, p: V2) -> &Cell{
		&self.map[p.x as usize + p.y as usize * self.width]
	}

	fn calc_clear_rows_score(&self, rows_cleared: usize) -> u32 {
		rows_cleared as u32 * 100 * if rows_cleared < 4 {1} else {2}
	}

	fn clear_rows_and_update_score(&mut self, points: &[V2]) {
		let mut unique_y_values = [0; 4];
		let mut y_count = 0u8;
		let mut rows_cleared = 0u8;
		for p in points {
			if !unique_y_values[..y_count as usize].contains(&p.y) {
				unique_y_values[y_count as usize] = p.y;
				y_count += 1;
				if self.check_row(p.y as usize) {
					self.clear_rows(p.y as usize, 1);
					rows_cleared += 1;
				}
			}
		}
		self.score += self.calc_clear_rows_score(rows_cleared as usize);
	}

	fn check_t_spin(&mut self) {
		let faller = self.faller();
		if faller.cell_value == Cell::Full(Shape::T) {
			let mut count = 0;
			let p = faller.pos;
			let offsets: [V2; 4] = [V2::new(-1, -1), V2::new(1, -1), V2::new(-1, 1), V2::new(1, 1)];
			for offset in offsets {
				let p = p + offset;
				if self.bounds_contain(p) && self.at(p).is_full() {
					count += 1;
				}
			}
			if count >= 3 {
				self.score += T_SPIN_SCORE;
			}
		}
	}

	// spawns a tetris piece at the top middle of the map
	fn spawn_random_piece(&mut self, index_of_piece_to_clear: usize) -> Result<(), SpawningError> {
		use SpawningError::*;

		self.clear_rows_and_update_score(&self.pieces[index_of_piece_to_clear].points.clone());

		let piece = Piece::new(V2::new(self.width as i32 / 2, 0), Cell::Full(rand::random())); // todo add rng
		if piece.points.iter().any(|p| self.at(*p).is_full()) {
			return Err(GameOver);
		}
		*self.faller_mut() = piece;
		self.update_ghost(false);
		self.insert_piece(FALLER_INDEX);
		self.reset_gravity_time();
		Ok(())
	}

	fn reset_gravity_time(&mut self) {
		self.next_gravity_time = self.now;
		self.update_next_gravity_time();
	}

	fn update_next_gravity_time(&mut self) {
		self.next_gravity_time += GAME_GRAVITY_INTERVAL;
	}

	fn spin(&mut self, rotate_left: bool) -> bool {
		if matches!(self.faller().cell_value.unwrap(), Shape::O) { // todo this is not a solution!!!!
			return false;
		}
		let mut rotated = if rotate_left {
			self.faller().rotated_left()
		} else {
			self.faller().rotated_right()
		};

		let mut v = V2::new(0, 0);
		for p in rotated.points {
			if p.x < 0 {
				v.x += 1;
			} else if p.x >= self.width as i32 {
				v.x -= 1;
			}
			if p.y < 0 {
				v.y += 1;
			} else if p.y >= self.height as i32 {
				v.y -= 1;
			}
		}

		if v != V2::new(0, 0) {
			rotated = rotated.translated(v);
		}

		// now check if it doesn't intersect with other things
//		let intersects: bool = rotated.points.iter().any(|p| self.at(*p).is_some() && self.piece.points.iter().find(|v| *p == **v).is_none());
		let intersects: bool = rotated.points.iter().any(|p| self.at(*p).is_full() && !self.faller().points.iter().any(|v| *p == *v));

		if intersects {
			// todo try different positions
		} else {
			self.update_piece(FALLER_INDEX, rotated, true);
//			*self.piece_mut() = rotated;
			return true;
		}
		false
	}

	// continuously moves a piece until it can't move anymore and return the number of moves
	fn throw_piece(&mut self, piece_index: usize, d: Direction) -> u32 {
		let mut moves = 0;
		let mut moved = self.pieces[piece_index].clone();
		while self.try_move_piece(&mut moved, d) {
			moves += 1;
		}
		self.pieces[piece_index] = moved;
		moves
	}

	fn bounds_contain(&self, V2{x,y}: V2) -> bool {
		x < self.width as i32 && x >= 0 && y >= 0 && y < self.height as i32
	}

	fn check_row(&self, y: usize) -> bool {
		let i = y * self.width;
		self.map[i..i + self.width].iter().all(Cell::is_full)
	}

	// clears a number of rows and drops the rows above it
	// should be called after check_rows() returns true
	fn clear_rows(&mut self, y: usize, rows_down: usize) {
		let thickness = self.width * rows_down;
		let i = y * self.width;
		self.map[i..i + thickness].iter_mut().for_each(|cell| cell.empty()); // clear rows
		self.map[..i + thickness].rotate_right(thickness); // drop gravity
	}

	fn handle_user_input(&mut self, action: GameInput) -> Result<(), SpawningError>{
		use Direction::*;
		use GameInput::*;

		self.display_changed = match action {
			SpinLeft => {
				if self.spin(true) {
					self.update_ghost(true);
					true
				} else {
					false
				}
			}
			SpinRight => {
				if self.spin(false) {
					self.update_ghost(true);
					true
				} else {
					false
				}
			}
			MoveLeft => {
				self.try_move_piece_and_update(FALLER_INDEX, Left)
			}
			MoveRight => {
				self.try_move_piece_and_update(FALLER_INDEX, Right)
			}
			MoveDown => {
				if self.try_move_piece_and_update(FALLER_INDEX, Down) {
					self.score += 1;
					self.reset_gravity_time();
					true
				} else {
					false
				}
			}
			DropDown => {
				self.score += 2 * (self.ghost().pos.y - self.faller().pos.y) as u32;
				self.update_piece(FALLER_INDEX, self.ghost().clone_as_full(), false);
				// todo the following two lines can be put into a function as they are also being used in apply_gravity
				self.spawn_piece_and_update_ghost()?;
				true
			}
		};

		Ok(())
	}

	// applies gravity and if fails because there is no piece or the piece can't fall, spawn a new one
	fn apply_gravity(&mut self) -> Result<(), SpawningError> {
		self.display_changed = true;
		if !self.try_move_piece_and_update(FALLER_INDEX, Direction::Down) {
			self.spawn_piece_and_update_ghost()?;
		}
		Ok(())
	}

	fn spawn_piece_and_update_ghost(&mut self)-> Result<(), SpawningError> {
		let r = self.spawn_random_piece(FALLER_INDEX);
		self.update_ghost(false);
		r
	}

	fn end_game(&mut self){
		self.game_over = true;
	}
}

//impl<R: Read, W: Write> ToString for Tetris<R, W> {
//	fn to_string(&self) -> String {
//		let mut output = String::new();
//		for (y, row) in self.map[..].chunks(self.width).enumerate() {
//			// let s = format!("{:02} ", y);
//			// output += &s;
//			// let (w, h) = (self.width - 1, self.height - 1);
//			for (x, cell) in row.iter().enumerate() {
//				match cell{
//					Cell::Full(shape) => {
//						output += shape.fg_color_str();
//						output += "██";
//					}
//					Cell::Ghost(_) => {
//						output += termion::color::White.fg_str();
//						output += "██";
//					}
//					Cell::Empty => {
//						output += if (y * self.width + x) % 2 == 0 { termion::color::Black.fg_str() } else { termion::color::LightBlack.fg_str() };
//						output += "░_";
//					}
//				};
//				// output += if block.is_some() {"██"} else {"░_"}; // • ░ ▒ ▓ █ ▀ ▄ ≡ ■  ⎸ ⎹ ⼕
//				// let s = if (x, y) == (0, 0) {"┌ "}
//				// 	else if (x, y) == (0, h) {"└ "}
//				// 	else if (x, y) == (w, h) {"┘ "}
//				// 	else if (x, y) == (w, 0) {"┐ "}
//				// 	else {"┼ "};
//				// let s = match (x, y) {
//				// 	(0, 0) => "┌ ",
//				// 	(0, h) => "└ ",
//				// 	(w, h) => "┘ ",
//				// 	(w, 0) => "┐ ",
//				// 	_ => "┼ ",
//				// };
//			}
//			output += "\n\r";
//		}
//		let score = self.score.to_string();
//		// let lines = std::iter::repeat("-").take((self.width * 2 - score.len()) / 2).collect::<String>();
//		let lines = "-".repeat(self.width * 2 - score.len() / 2);
//		output += &lines;
//		output += &score;
//		output += &lines;
//		output += "\r\n";
//		output
//	}
//}

impl<R: Read, W:Write> Tetris<R, W> {
	fn display(&mut self) {
		// • ░ ▒ ▓ █ ▀ ▄ ≡ ■  ⎸ ⎹ ⼕
		let f = &mut self.output;
		write!(f, "{}", termion::color::LightBlack.fg_str()).unwrap();
		for _ in 0..=self.width {
			write!(f, "▀▄").unwrap();
		}
		write!(f, "\n\r").unwrap();

		for row in self.map[..].chunks(self.width) {
			write!(f, "{}▓", termion::color::LightBlack.fg_str()).unwrap();
			for cell in row {
				write!(f, "{}", cell).unwrap();
			}
			write!(f, "{}▓\n\r", termion::color::LightBlack.fg_str()).unwrap();
		}

		let score = self.score.to_string();
		let lines = "-".repeat((self.width * 2 - score.len())/2);
		write!(f, "{}", termion::color::LightBlack.fg_str()).unwrap();
		for _ in 0..=self.width {
			write!(f, "▄▀").unwrap();
		}
		write!(f, "\n\r{l}{s}{l}\n\r", l=lines, s=score).unwrap();
	}
}

impl<R: Read, W: Write> Game for Tetris<R, W> {
	fn reset(&mut self) {
		self.game_over = true;
		self.score = 0;
		self.map.iter_mut().for_each(Cell::empty);
	}

	fn run(&mut self) -> bool {
		self.game_over = false;

		write!(self.output, "{}", termion::cursor::Hide).unwrap();
		self.output.flush().unwrap();

		self.spawn_random_piece(FALLER_INDEX).unwrap();
		self.update_ghost(false);
		self.insert_piece(FALLER_INDEX);

		self.now = time::Instant::now();
		self.start_time = self.now;
		self.next_frame_time = self.start_time;
		self.next_gravity_time = self.start_time;
		self.next_input_time = self.start_time;

		while !self.game_over {
			self.now = time::Instant::now(); // self.start_time.elapsed();

			// Get User-Input periodically
			if self.now >= self.next_input_time {
				let mut buf = [0u8; 1];
				self.input.read(&mut buf).expect("Reading input from the user must work!");
				use GameInput::*;

				let e = match buf[0] {
					// todo add arrow-keys
					b'w' | b'W' => self.handle_user_input(SpinLeft),
					b'q' | b'Q' => self.handle_user_input(SpinRight),
					b'a' | b'A' => self.handle_user_input(MoveLeft),
					b'd' | b'D' => self.handle_user_input(MoveRight),
					b's' | b'S' => self.handle_user_input(MoveDown),
					b' ' => self.handle_user_input(DropDown),
					b'x' | b'X' => {
						self.end_game();
						Ok(())
					}
					_ => Ok(())
				};
				if let Err(SpawningError::GameOver) = e {
					return false;
				}
				self.next_input_time += USER_INPUT_INTERVAL; // todo make update function
			}

			// Make the piece fall periodically
			if self.now >= self.next_gravity_time {
				if let Err(SpawningError::GameOver) = self.apply_gravity() {
					return false
				}
				self.update_next_gravity_time();
			}

			// Draw periodically
			if self.display_changed && self.now >= self.next_frame_time {
//				write!(self.output, "{}{}", termion::clear::All, self.to_string()).unwrap();
				write!(self.output, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
				self.display();
				self.output.flush().unwrap();

				self.next_frame_time += GAME_DRAW_INTERVAL; // todo make update function
				self.display_changed = false;
			}

			// Sleep until next event // todo test this
			use std::cmp::min;
			let soonest = min(self.next_frame_time, min(self.next_input_time, self.next_gravity_time));
			let actual_now = time::Instant::now();
			std::thread::sleep(soonest - actual_now);//std::time::Duration::from_millis(125));
		}
		true
	}

	fn score(&self) -> f32 {
		self.score as f32
	}
}

impl<R, W> Tetris<R, W> {
	// returns an iterator of all bools in the current state of the game.
	// it's lazy so if you update the game, the values iterated will change
	pub fn serialize(&self) -> impl Iterator<Item = bool> + '_ {
		self.map.iter().map(Cell::is_full)
	}
}

#[derive(Debug)]
enum SpawningError {
	GameOver,
}

#[derive(Copy, Clone)]
enum GameInput {
	MoveLeft, MoveRight, MoveDown, DropDown, SpinLeft, SpinRight
}
