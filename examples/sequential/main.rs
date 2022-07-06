use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use fif::compress;
use fif::Image;

const PNG_IN: &str = "/var/www/dev/rust/parallel-compression/in.png";
const PNG_OUT: &str = "/var/www/dev/rust/parallel-compression/out.png";
const FIF_OUT: &str = "/var/www/dev/rust/parallel-compression/out.fif";

fn decode_png<P: AsRef<Path>>(path: P) -> Result<(png::OutputInfo, Vec<u8>), std::io::Error> {
	let decoder = png::Decoder::new(File::open(path)?);
	let mut reader = decoder.read_info()?;
	let mut buf = vec![0; reader.output_buffer_size()];
	let info = reader.next_frame(&mut buf)?;
	buf.resize_with(info.buffer_size(), Default::default);
	Ok((info, buf))
}

fn encode_png<P: AsRef<OsStr>>(
	path: P,
	info: &png::OutputInfo,
	data: &[u8],
) -> Result<(), std::io::Error> {
	let path = Path::new(&path);
	let file = File::create(path)?;
	let ref mut w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, info.width, info.height);
	encoder.set_color(info.color_type);
	encoder.set_depth(info.bit_depth);
	let mut writer = encoder.write_header()?;

	writer.write_image_data(data)?;
	Ok(())
}

fn encode_fif<P: AsRef<OsStr>>(path: P, image: Image) -> Result<(), std::io::Error> {
	let path = Path::new(&path);
	let mut file = File::create(path)?;

	let mut bytes = Vec::new();
	for (_, values) in image.channels {
		let chunk = values
			.iter()
			.map(|value| (value * 255.) as u8)
			.collect::<Vec<u8>>();
		let diff = compress::diff(chunk, 16);

		for value in diff {
			match value {
				compress::Diff::Yes { average, diff } => {
					bytes.push(0b11110000);
					bytes.push(average);
					bytes.extend(diff.iter().map(|diff| {
						if diff.is_negative() {
							(diff.abs() as u8) | 0b10000000
						} else {
							(diff.abs() as u8) | 0b01000000
						}
					}))
				}
				compress::Diff::No(byte) => bytes.push(byte),
			}
		}
	}

	file.write_all(bytes.as_slice())?;
	Ok(())
}

fn read_fif<P: AsRef<Path>>(path: P) -> Result<Image, std::io::Error> {
	let file = File::open(path)?;
	let bytes = file.bytes().map(|byte| byte.unwrap());

	let mut image = Image {
		width: 1920,
		height: 1080,
		channels: HashMap::new(),
	};

	for byte in bytes {
		match byte {
			_ => {}
		}
	}

	Ok(image)
}

fn main() -> Result<(), std::io::Error> {
	let (info, png_in) = decode_png(PNG_IN)?;
	let image = Image::from_rgba(info.width, info.height, &png_in);

	encode_png(PNG_OUT, &info, png_in.as_slice())?;
	encode_fif(FIF_OUT, image)?;
	let _ = read_fif(FIF_OUT);
	Ok(())
}
