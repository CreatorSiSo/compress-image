use std::collections::HashMap;

use crate::write;

pub fn encode(width: u32, height: u32, channels: &HashMap<&[u8], Vec<u8>>) -> Vec<u8> {
	let mut encoded = Vec::new();

	write::header(
		&mut encoded,
		width,
		height,
		channels.iter().map(|(name, _)| *name).collect(),
	);
	write::values(&mut encoded, &[24, 38, 10, 2]);
	write::run(&mut encoded, 57, 200);
	write::diffs(&mut encoded, 254, &[-9, 0, 5, 16, -11]);

	encoded
}

#[cfg(test)]
mod test {
	use std::collections::HashMap;

	use crate::encode;

	#[test]
	fn encode_test() {
		let mut channels = HashMap::new();
		channels.insert(b"red".as_slice(), Vec::from([200; 200]));
		channels.insert(b"green".as_slice(), Vec::new());
		channels.insert(b"blue".as_slice(), Vec::new());
		channels.insert(b"alpha".as_slice(), Vec::new());

		let bytes = encode(1920, 1080, &channels);
	}
}
