use qango::board::{START, Board};
use qango::board::side::Side;
use ansi_escapes::*;
use std::fmt::{Display,Formatter};

pub struct Deep {
    pub none : u64,
    pub white : u64,
    pub black : u64,
}

impl Deep {
    pub fn start() {
        let mut start : Deep = Deep::default();

        start.move_white(&START);

        println!("n {} - w {} - b {}", start.none, start.white, start.black);
    }

    fn check_black(&mut self, board : &Board) -> Side {
        if board.won() == Side::BLACK {
            self.black += 1;
            return Side::BLACK;
        } else {
            return self.move_white(&board)
        }
    }

    fn move_white(&mut self, board : &Board) -> Side {
        print!("{}", ClearScreen);
        print!("{}", self);
        let turns = board.turns();

        if turns.len() == 0 {
            self.none += 1;
            return Side::NONE;
        }

        let mut result : Side = Side::BLACK;

        for i in turns.into_iter() {
            let nboard = board.turn(*i);

            match self.check_white(&nboard) {
                Side::WHITE => return Side::WHITE,
                Side::BLACK => (),
                Side::NONE => result = Side::NONE
            }
        }

        return result;
    }

    fn check_white(&mut self, board : &Board) -> Side {
        if board.won() == Side::WHITE {
            self.white += 1;
            return Side::WHITE;
        } else {
            return self.move_black(&board)
        }
    }

    fn move_black(&mut self, board : &Board) -> Side {
        print!("{}", ClearScreen);
        print!("{}", self);
        let turns = board.turns();

        if turns.len() == 0 {
            self.none += 1;
            return Side::NONE;
        }

        let mut result : Side = Side::WHITE;

        for i in turns.into_iter() {
            let nboard = board.turn(*i);

            match self.check_black(&nboard) {
                Side::BLACK => return Side::BLACK,
                Side::WHITE => (),
                Side::NONE => result = Side::NONE
            }
        }

        return result;
    }
}

impl Default for Deep {
    fn default() -> Deep {
        Deep {
            none : 0,
            white : 0,
            black : 0,
        }
    }
}

impl Display for Deep {
	fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
		write!(f, "none = {}\nwhite: {}\nblack: {}\n", self.none, self.white, self.black)
	}
}
