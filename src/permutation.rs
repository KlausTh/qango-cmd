use qango::board::encoding::*;

pub fn perm() {
	perm6();
	perm7();
}

pub fn perm6() {
	let p6 = permutations6x6();

	println!("\nPermutations with 6x6 board:");
	p6.iter().enumerate().for_each(|c| println!("{:02} Turn: {:>16} : 0x{:X} / ({} bits)",c.0,c.1,c.1,(u64::BITS - c.1.leading_zeros())));

	println!("================================");
	let sum = p6.iter().fold(0, |a,b| a+b);
	println!("sum = {:09} : 0x{:X} / ({} bits)",sum,sum,(u64::BITS - sum.leading_zeros()));
}

pub fn perm7() {
	let p7 : Vec<u128> = permutations7x7();

	println!("\nPermutations with 7x7 board:");
	p7.iter().enumerate().for_each(|c| println!("{:02} Turn: {:>22} : 0x{:X} / ({} bits)",c.0,c.1,c.1,(u128::BITS - c.1.leading_zeros())));

	println!("================================");
	let sum = p7.iter().fold(0, |a,b| a+b);
	println!("sum = {:09} : 0x{:X} / ({} bits)",sum,sum,(u128::BITS - sum.leading_zeros()));
}
