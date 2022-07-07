pub const MAGIC: &[u8; 4] = b"pif\0";

pub mod marker {
	/// ```ignore
	/// Byte[0] = 0b11111111
	/// Byte[1] = 0b11111111
	/// Byte[2..] = values
	/// ```
	pub const VALUE: &[u8; 2] = &[0b11111111, 0b11111111];

	/// TODO: Should this even exist?
	/// Its not very space efficient for a single channel.
	// pub const _INDEX: &[u8; 2] = &[0b00000000; 2];

	/// ```ignore
	/// Byte[0] = 0b11111110
	/// Byte[1] = 0b11111110
	/// Byte[2] = value
	/// Byte[3] = repeats
	/// ```
	pub const RUN: &[u8; 2] = &[0b11111110; 2];

	/// ```ignore
	/// Byte[0] = 0b11111100
	/// Byte[1] = 0b11111100
	/// Byte[2] = base
	/// Byte[3..] = diffs
	/// ```
	/// TODO: Smaller diff range and fit two diffs into one byte
	/// 0..255 => -127..128
	pub const DIFF: &[u8; 2] = &[0b11111100; 2];
}
