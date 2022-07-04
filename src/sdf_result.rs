use glam::Vec3;

use crate::Color;

pub struct SdfResult {
	pub range: f32,
	pub normal: Vec3,
	pub color: Color,
}
