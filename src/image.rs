use std::collections::HashMap;

pub struct Image<'a> {
	pub width: u32,
	pub height: u32,
	pub channels: HashMap<&'a str, Vec<f32>>,
}

impl Image<'_> {
	pub fn from_rgba(width: u32, height: u32, bytes: &Vec<u8>) -> Self {
		let mut red = Vec::default();
		let mut green = Vec::default();
		let mut blue = Vec::default();
		let mut alpha = Vec::default();

		for (index, byte) in bytes.iter().enumerate() {
			let value = (byte / 255) as f32;

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
				("red", red),
				("green", green),
				("blue", blue),
				("alpha", alpha),
			]),
		}
	}
}
