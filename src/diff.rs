#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Diff {
	average: u8,
	diff: Vec<u8>, // TODO: Make this signed
}

impl Diff {
	fn _new(average: u8, diff: Vec<u8>) -> Self {
		Self { average, diff }
	}
}

fn _diff_compress(bytes: Vec<u8>, max_diff: u8) -> Vec<Vec<u8>> {
	let result = bytes.iter().cloned().enumerate().fold(
		(vec![vec![]], 0_usize),
		|(mut sequences, sequence_start), (index, byte)| {
			let sequence = &bytes[sequence_start..index];

			if !sequence.is_empty() {
				println!("\nSequence: {:?}", sequence);

				if let Some(next_byte) = bytes.get(index) {
					println!("Next Byte: {:?}", next_byte);

					let sum = sequence.iter().fold(0_usize, |acc, v| acc + *v as usize);
					let avrg = (sum / sequence.len()) as u8;
					println!("Avrg: {:?}", avrg);

					let next_diff = avrg.abs_diff(*next_byte);
					let diff_too_big = next_diff > max_diff;
					println!("Next Diff: {:?} [{}]", next_diff, diff_too_big);

					if diff_too_big {
						println!("-> Creating new sequence");
						sequences.push(Vec::new());
						sequences.last_mut().unwrap().push(byte);
						return (sequences, index);
					}
				}
			}

			sequences.last_mut().unwrap().push(byte);
			(sequences, sequence_start)
		},
	);

	println!("\n=> {:?}", result);
	result.0
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn plateaus_test() {
		/*  avrg   =     [53                        ][205][255]*/
		/*  diffs  =     [ 1,  3, -5, -3, -1, -4,  9][  0][  0]*/
		let values = vec![54, 56, 48, 50, 52, 49, 62, 205, 255];
		let compressed = _diff_compress(values, 10);

		assert_eq!(
			compressed,
			vec![vec![54, 56, 48, 50, 52, 49], vec![62], vec![205], vec![255]]
		)
	}
}
