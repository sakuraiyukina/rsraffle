use rand::seq::SliceRandom;

const FIRST_PRIZE: usize = 1;
const SECOND_PRIZE : usize = 2;

pub fn getuid() {
	let mut uids = [
		"f37244e", "c761a3e", "511d219", "630799b", "f19226a", "a2a0341", "f29aa2e", "1c914c9",
		"c93bd1d", "77114e0", "cdb526d", "bfdbb82", "581cdec", "eee254c", "0f2d6b2", "444c5fd",
		"77d6427", "06bf7cf", "f678759", "a5434fd", "e6d4f9e", "676e741", "cf32077", "b7faf8f",
		"500e6c5", "16f4bbd", "2c50742"
	];
	println!("{:?}", uids);

	for i in 0..3 {
		uids.shuffle(&mut rand::thread_rng());
		println!("The number of times array after out of order:{} {:?}", i + 1, uids);
	}

	let winners= uids.choose_multiple(&mut rand::thread_rng(), FIRST_PRIZE + SECOND_PRIZE).cloned().collect::<Vec<_>>();

	println!("First prize UID(s): {}", winners[0]);
	println!("Second prize UID(s): {}, {}", winners[1], winners[2]);
}
