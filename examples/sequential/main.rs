use std::ffi::OsStr;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use fif::Image;

const PNG_IN: &str = "/var/www/dev/rust/parallel-compression/in.png";
const PNG_OUT: &str = "/var/www/dev/rust/parallel-compression/out.png";

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
	info: png::OutputInfo,
	data: Vec<u8>,
) -> Result<(), std::io::Error> {
	let path = Path::new(&path);
	let file = File::create(path)?;
	let ref mut w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, info.width, info.height);
	encoder.set_color(info.color_type);
	encoder.set_depth(info.bit_depth);
	let mut writer = encoder.write_header()?;

	writer.write_image_data(&data)?;
	Ok(())
}

fn main() -> Result<(), std::io::Error> {
	let (info, png_in) = decode_png(PNG_IN)?;
	let _image = Image::from_rgba(info.width, info.height, &png_in);
	encode_png(PNG_OUT, info, png_in)?;

	Ok(())
}
