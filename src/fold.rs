// pub fn fold(bytes: Vec<u8>) -> Option<(u8, u8)> {
// 	let iter = bytes.into_iter();
// 	let max = iter.clone().max()?;
// 	let min = iter.clone().min()?;

// 	Some((min, max))
// }

// match min - max {
// 	0..=127 => {}
// 	128..=255 => {}
// }

pub fn fold(bytes: Vec<u8>) -> Option<Vec<u8>> {
	// let iter = bytes.iter();
	// let mut max = 0;

	// for (index, byte) in iter.enumerate() {
	// 	max = max.max(*byte);

	// 	if max > 127 {
	// 		println!("{:?}", &bytes[0..index]);
	// 	}
	// }

	// let mut iter = bytes.into_iter().peekable();

	// while iter.peek().is_some() {
	// 	// let chunk: Vec<_> = iter.by_ref().take_while(|byte| (byte <= &127)).collect();
	// 	let chunk: Vec<_> = iter
	// 		.by_ref()
	// 		.reduce(|accum, current| (accum.max(current), current));
	// 	println!("{:?}", chunk);
	// }

	let output: Vec<Vec<u8>> = bytes.into_iter().fold(Vec::new(), |mut acc, item| {
		if item == 0 || acc.is_empty() {
			acc.push(Vec::new());
		}
		acc.last_mut().unwrap().push(item);
		acc
	});

	println!("{:?}", output);

	None
}

#[cfg(test)]
mod test {
	use super::fold;

	#[test]
	fn fold_test() {
		let data = vec![0, 50, 100, 126, 120, 120, 0, 255, 255, 100, 126, 120];

		if let Some(folded) = fold(data) {
			println!("{:?}", folded);
		}
	}
}
