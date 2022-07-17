use glam::Vec3;

use crate::{SdfResult, Color, Sdf};

pub struct Plane {
	color: Color,
}

impl Plane {
	pub fn new(color: Color) -> Self {
		Self { color }
	}
}

impl Sdf for Plane {
	fn check(&self, pos: Vec3) -> SdfResult {
		SdfResult { range: pos.y, normal: Vec3::Y, color: self.color }
	}
}
