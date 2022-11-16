extern crate termion;

//use termion::event::Key;
//use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::stdout;


mod tetris;
use crate::tetris::game::Game;


fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");
	let stdout = stdout().into_raw_mode().unwrap();
	let stdin = termion::async_stdin();

	let mut game = tetris::Tetris::new(10, 20, stdout, stdin);
	game.reset();
	game.run();
}
