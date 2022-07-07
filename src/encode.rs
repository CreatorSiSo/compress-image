use std::collections::HashMap;

use crate::consts;

pub fn write_header(
	buf: &mut Vec<u8>,
	width: u32,
	height: u32,
	_channels: &HashMap<String, Vec<f32>>,
) {
	buf.extend_from_slice(consts::MAGIC);
	buf.extend_from_slice(&width.to_be_bytes());
	buf.extend_from_slice(&height.to_be_bytes());
	// TODO Write list of channels
}

pub fn write_values(buf: &mut Vec<u8>, values: &[u8]) {
	buf.extend_from_slice(consts::marker::VALUE);
	buf.extend_from_slice(values);
}

/// TODO ensure that nothing breaks if value and number of repeats are the same
pub fn write_run(buf: &mut Vec<u8>, value: u8, repeats: u8) {
	buf.push(value);
	buf.extend_from_slice(consts::marker::RUN);
	buf.push(repeats);
}
