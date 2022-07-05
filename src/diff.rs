fn _plateaus(bytes: Vec<u8>, _max_dist: u8) -> Vec<u8> {
	let result = bytes.into_iter().fold(
		(0, 0 as usize, Vec::new()),
		|(len, sum, mut plateaus), current| {
			if len < 1 {
				return (1, current as usize, plateaus);
			}

			let average = sum / len;
			plateaus.push(average.abs_diff(current.into()));
			(len + 1, sum + current as usize, plateaus)
		},
	);

	println!("{:?}", result);

	// let mut accum: u8 = 0;
	// let mut len: usize = 0;

	// for byte in bytes {
	// 	let average = (accum as usize / len) as u8;
	// 	if accum.abs_diff(byte) <= max_distance {
	// 		accum = byte;
	// 		len = 1;
	// 	}

	// 	accum += byte;
	// 	len += 1;
	// }

	Vec::default()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn plateaus_test() {
		_plateaus(vec![54, 56, 48, 50, 52, 49, 62, 205, 255], 6);
	}
}
