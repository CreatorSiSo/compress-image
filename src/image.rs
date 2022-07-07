use std::collections::HashMap;

use crate::write;

#[derive(Debug)]
pub struct Image {
	pub width: u32,
	pub height: u32,
	pub channels: HashMap<String, Vec<f32>>,
}

impl Image {
	pub fn from_bytes_rgba(width: u32, height: u32, bytes: &[u8]) -> Self {
		let mut red = Vec::default();
		let mut green = Vec::default();
		let mut blue = Vec::default();
		let mut alpha = Vec::default();

		for (index, byte) in bytes.iter().enumerate() {
			let value = *byte as f32 / 255.;

			match index % 4 {
				0 => red.push(value),
				1 => green.push(value),
				2 => blue.push(value),
				3 => alpha.push(value),
				_ => {}
			}
		}

		Self {
			width,
			height,
			channels: HashMap::from([
				("red".into(), red),
				("green".into(), green),
				("blue".into(), blue),
				("alpha".into(), alpha),
			]),
		}
	}

	pub fn encode(&self) -> Vec<u8> {
		let mut encoded = Vec::new();

		write::header(&mut encoded, self.width, self.height, &self.channels);
		write::values(&mut encoded, &[24, 38, 10, 2]);
		write::run(&mut encoded, 57, 200);
		write::diffs(&mut encoded, 254, &[-9, 0, 5, 16, -11]);

		encoded
	}
}
