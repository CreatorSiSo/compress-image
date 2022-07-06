#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Diff {
	Yes {
		average: u8,
		diff: Vec<i8>, // TODO: Make this signed
	},
	No(u8),
}

// TODO: Should return Vec<&[u8]>
fn chunks_by_diff(bytes: Vec<u8>, max_diff: u8) -> Vec<Vec<u8>> {
	bytes
		.iter()
		.cloned() // TODO: Remove
		.enumerate()
		.fold(
			(vec![vec![]], 0_usize),
			|(mut chunks, chunk_start), (index, byte)| {
				let chunk = &bytes[chunk_start..index];

				if !chunk.is_empty() {
					if let Some(next_byte) = bytes.get(index) {
						let sum = chunk.iter().fold(0, |acc, v| acc + *v as usize);
						let avrg = (sum / chunk.len()) as u8;

						let next_diff = avrg.abs_diff(*next_byte);
						let diff_too_big = next_diff > max_diff;

						if diff_too_big {
							chunks.push(Vec::new());
							chunks.last_mut().unwrap().push(byte);
							return (chunks, index);
						}
					}
				}

				chunks.last_mut().unwrap().push(byte);
				(chunks, chunk_start)
			},
		)
		.0
}

pub fn diff(bytes: Vec<u8>, max_diff: u8) -> Vec<Diff> {
	let chunks = chunks_by_diff(bytes, max_diff);

	let mut result = Vec::new();
	for chunk in chunks {
		if chunk.len() == 1 {
			result.push(Diff::No(chunk[0]));
			continue;
		}

		let average = (chunk.iter().fold(0, |acc, v| acc + *v as usize) / chunk.len()) as u8;
		// TODO: Make diffe signed
		// chunk.iter().map(|byte| byte - average).collect(),
		let diff = chunk
			.iter()
			.map(|byte| (*byte as i16 - average as i16) as i8)
			.collect();
		result.push(Diff::Yes { average, diff });
	}
	result
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn plateaus_test() {
		/*  avrg   =     [53                        ][205][255]*/
		/*  diffs  =     [ 1,  3, -5, -3, -1, -4,  9][  0][  0]*/
		let values = vec![54, 56, 48, 50, 52, 49, 62, 205, 255];
		let compressed = diff(values, 10);

		assert_eq!(
			compressed,
			vec![
				Diff::Yes {
					average: 51,
					diff: vec![3, 5, -3, -1, 1, -2]
				},
				Diff::No(62),
				Diff::No(205),
				Diff::No(255)
			]
		)
	}
}
