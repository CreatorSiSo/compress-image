use crate::consts;

pub fn header(buf: &mut Vec<u8>, width: u32, height: u32, channel_names: Vec<&[u8]>) {
	buf.extend_from_slice(consts::MAGIC);
	buf.extend_from_slice(&width.to_be_bytes());
	buf.extend_from_slice(&height.to_be_bytes());

	// TODO Ensure that nothing breaks if the same byte is repeated
	for channel_name in channel_names {
		buf.extend_from_slice(channel_name);
		// TODO Use seperation markers to know when a name ends
	}
}

pub fn values(buf: &mut Vec<u8>, values: &[u8]) {
	buf.extend_from_slice(consts::marker::VALUE);
	buf.extend_from_slice(values);
}

/// TODO Ensure that nothing breaks if value and number of repeats are the same
pub fn run(buf: &mut Vec<u8>, value: u8, repeats: u8) {
	buf.extend_from_slice(consts::marker::RUN);
	buf.push(value);
	buf.push(repeats);
}

pub fn diffs(buf: &mut Vec<u8>, base: u8, diffs: &[i8]) {
	let diff_bytes: Vec<u8> = diffs.iter().map(|v| (v + i8::MIN) as u8).collect();
	buf.extend_from_slice(consts::marker::DIFF);
	buf.push(base);
	buf.extend_from_slice(diff_bytes.as_slice())
}

#[cfg(test)]
mod test {
	use crate::write;

	#[test]
	fn write_header() {
		let mut buf = Vec::new();
		// TODO Test writing channel names.
		write::header(&mut buf, 1821, 1821, vec![]);

		assert_eq!(
			buf,
			vec![
				112, 105, 102, 0, // pif\0
				0x00, 0x00, 0x07, 0x1d, // 1821
				0x00, 0x00, 0x07, 0x1d, // 1821
			]
		)
	}

	#[test]
	fn write_values() {
		let mut buf = Vec::new();
		write::values(&mut buf, &[222, 190, 2, 45, 21]);

		assert_eq!(
			buf,
			vec![
				0b11111111, 0b11111111, // marker
				222, 190, 2, 45, 21, // values
			]
		)
	}

	#[test]
	fn write_run() {
		let mut buf = Vec::new();
		write::run(&mut buf, 111, 255);

		assert_eq!(
			buf,
			vec![
				0b11111110, 0b11111110, // marker
				111,        // value
				255,        // repeats
			]
		)
	}

	#[test]
	fn write_diffs() {
		let mut buf = Vec::new();
		write::diffs(&mut buf, 100, &[-9, 0, 5, 16, -11]);

		assert_eq!(
			buf,
			vec![
				0b11111100, 0b11111100, // marker
				100,        // base
				119, 128, 133, 144, 117 // diffs
			]
		)
	}
}
