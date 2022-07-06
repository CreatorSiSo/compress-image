use std::collections::HashMap;

#[derive(Debug)]
pub struct Image {
	pub width: u32,
	pub height: u32,
	pub channels: HashMap<String, Vec<f32>>,
}

impl Image {
	pub fn from_rgba(width: u32, height: u32, bytes: &[u8]) -> Self {
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
}
