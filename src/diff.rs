#[derive(Debug, Default)]
struct Diff {
	average: u8,
	diff: Vec<u8>, // TODO: Make this signed
}

impl Diff {
	fn new(average: u8, diff: Vec<u8>) -> Self {
		Self { average, diff }
	}
}

fn _diff_compress(bytes: Vec<u8>, max_dist: u8) -> Vec<u8> {
	println!("Bytes: {:?}\n", bytes);

	let result = bytes.into_iter().fold(
		(0_usize, 0_usize, 0_u8, Vec::default()),
		|(len, sum, avrg, mut plateaus /*, diffs */), current| {
			if len < 1 {
				return (1, current.into(), current, plateaus);
			}

			let diff = avrg.abs_diff(current);
			if diff > max_dist {
				plateaus.push(Diff::new(avrg, Vec::new()));
				return (1, current.into(), current, plateaus);
			}

			let new_avrg = (sum / len) as u8;
			println!("Avrg: {new_avrg}, Current: {current} Diff:{diff}");
			// plateaus.last_mut().unwrap().push(new_average);
			(len + 1, sum + current as usize, new_avrg, plateaus)

			// plateaus.push(average.abs_diff(current.into()));
		},
	);

	println!("\n{:?}\n", result);

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
		_diff_compress(vec![54, 56, 48, 50, 52, 49, 62, 205, 255], 10);
	}
}
