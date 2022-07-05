use glam::Vec3;

use crate::{SdfResult, Color, Sdf};

pub struct Sphere {
	radius: f32,
	color: Color,
}

impl Sphere {
	pub fn new(radius: f32, color: Color) -> Self {
		Self { radius, color }
	}
}

impl Sdf for Sphere {
	fn check(&self, pos: Vec3) -> SdfResult {
		SdfResult {
			range: pos.length() - self.radius,
			normal: pos.normalize_or_zero(),
			color: self.color,
		}
	}
}
