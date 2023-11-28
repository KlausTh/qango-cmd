
extern crate qango;
extern crate clap;

mod analyse;
mod index;
mod deep;
mod play;
mod permutation;

use clap::*;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug)]
enum Cmd {
	Run,
	Demo,
	Deep,
	Perm,
	Analyse,
	Index,
	Version,
	Help,
}

impl FromStr for Cmd {
	type Err = ();

	fn from_str(src: &str) -> Result<Cmd, ()> {
		match src.to_lowercase().as_str() {
			"run"     => Ok(Cmd::Run),
			"demo"    => Ok(Cmd::Demo),
			"deep"    => Ok(Cmd::Deep),
			"perm"    => Ok(Cmd::Perm),
			"analyse" => Ok(Cmd::Analyse),
			"index"   => Ok(Cmd::Index),
			"version" => Ok(Cmd::Version),
			"help"    => Ok(Cmd::Help),
			_         => Err(())
		}
	}
}

#[derive(Debug)]
struct Configuration {
	cmd : Cmd,
	debug : bool,
	round : usize,
}

static CONFIG : OnceLock<Configuration> = OnceLock::new();

fn main() {
	let config = Configuration::get();

	match config.cmd {
		Cmd::Run     => play::play(),
		Cmd::Demo    => play::demo(),
		Cmd::Deep    => deep::Deep::start(),
		Cmd::Perm    => permutation::perm(),
		Cmd::Analyse => analyse::analyse(),
		Cmd::Index   => index::index(),
		Cmd::Version => print_version(),
		_            => print_usage(),
	}
}

impl Configuration {
	pub fn get() -> &'static Self {
		CONFIG.get_or_init(|| Self::init())
	}

	fn init() -> Self {
		let mut config = Self {cmd: Cmd::Help, debug: false, round: 0};

		parse_args(&mut config);

		return config;
	}

	pub fn is_debug(&self) -> bool { self.debug }

	pub fn get_round(&self) -> usize { self.round }
}

fn parse_args(config : &mut Configuration) {
	let matches = Command::new("qango-cmd")
		.author("Klaus Thomas <klaus@itzgrund.net>")
		.version(env!("CARGO_PKG_VERSION"))
		.about("command line tool for board game qango")
		.arg(Arg::new("debug")
			.long("debug")
			.short('d')
			.action(clap::ArgAction::SetTrue)
			.help("turns on debugging mode"))
		.subcommand(Command::new("perm")
			.about("list all permutations"))
		.subcommand(Command::new("analyse")
			.about("analyse complete game"))
		.subcommand(Command::new("index")
			.about("read an index file")
			.arg(Arg::new("output")
				.short('o')
				.long("output")))
		.subcommand_required(true)
		.get_matches();

	config.debug = matches.get_flag("debug");
	config.cmd = Cmd::from_str(matches.subcommand().unwrap().0).unwrap_or(Cmd::Help);
}

fn print_version() {
	println!("Version: {}", env!("CARGO_PKG_VERSION"));
}

fn print_usage() {
	println!("usage: qango [run|demo|perm|analyse|index|version|help]");
}
