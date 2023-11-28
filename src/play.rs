
use qango::Game;
use qango::board::side::Side;
use qango::player::simple::Simple;
use qango::player::deep::Deep;
use std::time::Instant;

pub fn play() {
	let p1 = Box::new(Simple::new());
	let p2 = Box::new(Deep::new(2));

	let mut game = Game::new(p1, p2);
	
	println!("{}", game.get_board());
	while game.step() == Side::NONE {
		println!("{}", game.get_board())
	}
	println!("{}", game.get_board());
}

pub fn demo() {
	let p1 = Box::new(Deep::new(2));
	let p2 = Box::new(Deep::new(3));

	let mut game = Game::new(p1, p2);
	
	println!("{}", game.get_board());
	let mut start = Instant::now();
	while game.step() == Side::NONE {
		println!("player takes {} ms", start.elapsed().as_millis());
		start = Instant::now();
		println!("{}", game.get_board())
	}
	println!("player takes {} ms", start.elapsed().as_millis());
	println!("{}", game.get_board());
}
