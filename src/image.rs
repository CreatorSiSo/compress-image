use std::collections::HashMap;

use crate::consts::{self, marker};

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

		// Write Header
		{
			encoded.extend_from_slice(consts::MAGIC);
			encoded.extend_from_slice(&self.width.to_be_bytes());
			encoded.extend_from_slice(&self.height.to_be_bytes());
			// TODO Write list of channels
		}

		{
			encoded.extend_from_slice(marker::VALUE);
			encoded.push(24);
			encoded.push(38);
			encoded.push(10);
			encoded.push(2);
		}

		{
			encoded.push(254);
			encoded.extend_from_slice(marker::RUN);
			encoded.push(200);
		}

		{
			encoded.push(254);
			encoded.extend_from_slice(marker::DIFF);
			encoded.push(100);
			encoded.push(5);
			encoded.push(55);
		}

		encoded
	}
}
