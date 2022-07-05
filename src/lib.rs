mod diff;
mod fold;
mod image;

pub use image::*;

pub mod compress {
	pub use super::diff::*;
	pub use super::fold::*;
}
