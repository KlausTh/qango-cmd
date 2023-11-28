
use rayon::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter,Error,Write};
use std::sync::{Arc, Mutex};
use qango::board::encoding::*;
use qango::board::Board;
use super::Configuration;

pub fn analyse() {
	let config = Configuration::get();

	println!("offsets = {:?}", OFFSET);
	//println!("write round {}", config.get_round());

	iterate(35);
}

fn iterate(round : usize) -> usize {
	let fwin : File = File::create(&Path::new("round_35.dat")).unwrap();
	let writer = Arc::new(Mutex::new(BufWriter::new(&fwin)));

	println!("iterate positions = {:?} -> {:?}", OFFSET[round],OFFSET[round+1]);

	let size = (OFFSET[round]..OFFSET[round+1])
		.into_par_iter()
		.inspect(|i| status(&i,round))
		.map(|i| (i, Board::decode(i).unwrap()))
		.filter(|(_i,b)| b.is_running())
		.filter(|(i,b)| check_successors(*i, &b, Arc::clone(&writer)))
		.count();

	let _ = writer.lock().unwrap().flush();
	
	return size;
}

fn status(i : &u64, round : usize) {
	if i % 4096 == 0 {
		print!("\rturn {:16x} / {:16x}",i,OFFSET[round+1]);
	}
}

fn check_successors(i : u64, board : &Board, writer : Arc<Mutex<BufWriter<&File>>>) -> bool {
	let next = board.get_next();
	let successors : Vec<Board> = board.turns().into_iter().map(|t| board.turn(*t)).collect();

	if successors.iter().any(|b| b.won() == next) {
		let _ = save_winner(i, writer);

		return true;
	}

	// todo next level...

	return false;
}

fn save_winner(i : u64, mutex : Arc<Mutex<BufWriter<&File>>>) -> Result<usize, Error> {
	let bytes = i.to_ne_bytes();
	let mut writer = mutex.lock().unwrap();

	writer.write(&bytes)
}
