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
	// write::values(&mut encoded, &[24, 38, 10, 2]);
	// write::run(&mut encoded, 57, 200);
	// write::diffs(&mut encoded, 254, &[-9, 0, 5, 16, -11]);

	encoded
}

#[cfg(test)]
mod test {
	use std::collections::HashMap;

	use crate::encode;

	#[test]
	fn encode_test() {
		let mut channels = HashMap::new();
		channels.insert(b"red".as_slice(), Vec::from([200; 0]));
		channels.insert(b"green".as_slice(), Vec::new());
		channels.insert(b"blue".as_slice(), Vec::new());
		channels.insert(b"alpha".as_slice(), Vec::new());

		println!("{:?}", channels);

		let bytes = encode(1920, 1080, &channels);

		assert_eq!(
			bytes,
			vec![
				112, 105, 102, 0, // pif\0
				0x00, 0x00, 0x07, 0x80, // 1920
				0x00, 0x00, 0x04, 0x38, // 1080
				// TODO Figure out how to test the channel names encoding
				// TODO because it currently spits out somewhat random bytes which
				// TODO are probably the correct same string
				// TODO (tldr: bytes change form test to test but are prob. correct)
				b'r', b'e', b'd', // red
				b'g', b'r', b'e', b'e', b'n', // green
				b'b', b'l', b'u', b'e', // blue
				b'a', b'l', b'p', b'h', b'a', // alpha
			]
		)
	}
}
