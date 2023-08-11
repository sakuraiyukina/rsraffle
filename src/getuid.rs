use std::porcess;
use rand::seq::SliceRandom;

const FIRST_PRIZE: usize = 1;
const SECOND_PRIZE : usize = 2;

pub fn getuid() {
	let mut uids = [];
	println!("{:?}", uids);

	if uids.len() > FIRST_PRIZE + SECOND_PRIZE {process::exit(1)};

	for i in 0..3 {
		uids.shuffle(&mut rand::thread_rng());
		println!("The number of times array after out of order:{} {:?}", i + 1, uids);
	}

	let winners= uids.choose_multiple(&mut rand::thread_rng(), FIRST_PRIZE + SECOND_PRIZE).cloned().collect::<Vec<_>>();

	println!("First prize UID(s): {}", winners[0]);
	println!("Second prize UID(s): {}, {}", winners[1], winners[2]);
}
